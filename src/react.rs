use std::{collections::HashMap, hash::Hash};

#[allow(dead_code)]
struct Generator(u32);

#[allow(dead_code)]
impl Generator {
      fn new() -> Self { Generator(0) }

      fn gen(&mut self) -> u32 {
            let id = self.0;
            self.0 = id + 1;
            id
      }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct InputCellId(u32);

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ComputeCellId(u32);

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct CallbackId(u32);

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
      Input(InputCellId),
      Compute(ComputeCellId),
      Callback(CallbackId),
}

impl Hash for CellId {
      fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            match self {
                  CellId::Input(x) => x.hash(state),
                  CellId::Compute(x) => x.hash(state),
                  CellId::Callback(x) => x.hash(state),
            };
      }
}

#[allow(dead_code)]
pub enum Cell<T> {
      Input(T),
      Compute(Vec<T>),
      Callback(Box<dyn Fn(&[T]) -> T>),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
      NonexistentCell,
      NonexistentCallback,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Publisher<T> {
      value: T,
      subs:  Vec<CellId>,
}

impl<T> Publisher<T> {
      fn new(value: T) -> Self {
            Publisher {
                  value,
                  subs: vec![],
            }
      }
}

#[allow(dead_code)]
pub struct Subscriber<T> {
      pubs: Vec<CellId>,
      func: Box<dyn Fn(&[T]) -> T>,
}

impl<T> Subscriber<T> {
      fn new(func: Box<dyn Fn(&[T]) -> T>, pubs: Vec<CellId>) -> Self { Subscriber { func, pubs } }
}

#[allow(dead_code)]
pub struct Reactor<T> {
      generator:   Generator,
      publishers:  HashMap<CellId, Publisher<T>>,
      subscribers: HashMap<CellId, Subscriber<T>>,
      callbacks:   HashMap<CellId, Box<dyn FnMut(T) + 'static>>,
}

#[allow(dead_code)]
impl<T: Copy + PartialEq> Reactor<T> {
      pub fn new() -> Self {
            let generator = Generator::new();
            let publishers = HashMap::<CellId, Publisher<T>>::new();
            let subscribers = HashMap::<CellId, Subscriber<T>>::new();
            let callbacks = HashMap::<CellId, Box<dyn FnMut(T)>>::new();

            Reactor {
                  generator,
                  publishers,
                  subscribers,
                  callbacks,
            }
      }

      pub fn create_input(&mut self, initial: T) -> InputCellId {
            let unique = self.generator.gen();
            self.create_input_with_id(unique, initial)
      }

      fn create_input_with_id(&mut self, id: u32, initial: T) -> InputCellId {
            let publisher = Publisher::<T>::new(initial);
            let inputcell = InputCellId(id);
            self.publishers.insert(CellId::Input(inputcell), publisher);
            inputcell
      }

      fn deps_to_publishers(&mut self, deps: &[CellId]) -> Result<Vec<Publisher<T>>, CellId> {
            let mut values: Vec<Publisher<T>> = Vec::with_capacity(deps.len());
            for dep in deps {
                  match self.publishers.get_mut(dep) {
                        Some(publisher) => values.push(publisher.clone()),
                        None => return Err(*dep),
                  };
            }

            Ok(values)
      }

      pub fn create_compute<F>(
            &mut self,
            dependencies: &[CellId],
            compute_func: F,
      ) -> Result<ComputeCellId, CellId>
      where
            F: Fn(&[T]) -> T + 'static,
      {
            let generated_id = self.generator.gen();
            let compute_cell_id = ComputeCellId(generated_id);
            let cell_id = CellId::Compute(compute_cell_id);

            let mut parameters: Vec<T> = Vec::with_capacity(dependencies.len());

            for dep in dependencies {
                  match self.publishers.get_mut(dep) {
                        Some(publisher) => {
                              publisher.subs.push(cell_id);
                              parameters.push(publisher.value);
                        },
                        None => return Err(*dep),
                  }
            }

            self.publishers
                  .insert(cell_id, Publisher::new(compute_func(&parameters)));
            self.subscribers.insert(
                  cell_id,
                  Subscriber::new(Box::new(compute_func), dependencies.to_vec()),
            );

            Ok(compute_cell_id)
      }

      pub fn value(&self, id: CellId) -> Option<T> {
            match self.publishers.get(&id) {
                  Some(publisher) => Some(publisher.value),
                  None => None,
            }
      }

      pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
            let cell_id = CellId::Input(id);
            match self.publishers.get_mut(&cell_id) {
                  Some(publisher) => {
                        publisher.value = new_value;
                        self.update_subscribers(&cell_id);
                        true
                  },
                  None => false,
            }
      }

      fn update_subscribers(&mut self, id: &CellId) {
            match self.subscribers.get_mut(id) {
                  Some(subscriber) => {
                        let mut parameters: Vec<T> = Vec::with_capacity(subscriber.pubs.len());

                        for pub_id in &subscriber.pubs {
                              match self.publishers.get_mut(pub_id) {
                                    Some(publisher) => {
                                          parameters.push(publisher.value);
                                    },
                                    None => panic!("Publisher not found"),
                              }
                        }

                        let new_value = (subscriber.func)(&parameters);

                        match self.publishers.get_mut(id) {
                              Some(publisher) => {
                                    publisher.value = new_value;
                              },
                              None => panic!("Publisher not found"),
                        }
                        self.update_subscribers(id);
                  },
                  None => (),
            }
      }

      // Adds a callback to the specified compute cell.
      //
      // Returns the ID of the just-added callback, or None if the cell doesn't
      // exist.
      //
      // Callbacks on input cells will not be tested.
      //
      // The semantics of callbacks (as will be tested):
      // For a single set_value call, each compute cell's callbacks should each be
      // called:
      // * Zero times if the compute cell's value did not change as a result of the
      //   set_value call.
      // * Exactly once if the compute cell's value changed as a result of the
      //   set_value call. The value passed to the callback should be the final value
      //   of the compute cell after the set_value call.
      pub fn add_callback<F: FnMut(T) + 'static>(
            &mut self,
            id: ComputeCellId,
            callback: F,
      ) -> Option<CallbackId> {
            let cell_id = CellId::Compute(id);
            match self.publishers.get_mut(&cell_id) {
                  Some(publisher) => {
                        let callback_id = CallbackId(self.generator.gen());
                        let callback = Box::new(callback);
                        let callback_cell_id = CellId::Callback(callback_id);
                        self.callbacks.insert(callback_cell_id, callback);
                        publisher.subs.push(callback_cell_id);
                        Some(callback_id)
                  },
                  None => None,
            }
      }

      // Removes the specified callback, using an ID returned from add_callback.
      //
      // Returns an Err if either the cell or callback does not exist.
      //
      // A removed callback should no longer be called.
      pub fn remove_callback(
            &mut self,
            _cell: ComputeCellId,
            _callback: CallbackId,
      ) -> Result<(), RemoveCallbackError> {
            unimplemented!()
      }
}

#[cfg(test)]
mod tests {

      use super::*;
      use std::cell::Cell as C;
      #[test]
      fn input_cells_have_a_value() {
            let mut reactor = Reactor::new();
            let input = reactor.create_input(10);
            assert_eq!(reactor.value(CellId::Input(input)), Some(10));
      }

      #[test]
      fn an_input_cells_value_can_be_set() {
            let mut reactor = Reactor::new();
            let input = reactor.create_input(4);
            assert!(reactor.set_value(input, 20));
            assert_eq!(reactor.value(CellId::Input(input)), Some(20));
      }

      #[test]
      fn error_setting_a_nonexistent_input_cell() {
            let mut dummy_reactor = Reactor::new();
            let input = dummy_reactor.create_input(1);
            assert!(!Reactor::new().set_value(input, 0));
      }

      #[test]
      fn compute_cells_calculate_initial_value() {
            let mut reactor = Reactor::new();
            let input = reactor.create_input(1);
            let output = reactor
                  .create_compute(&[CellId::Input(input)], |v| v[0] + 1)
                  .unwrap();
            assert_eq!(reactor.value(CellId::Compute(output)), Some(2));
      }

      #[test]
      fn compute_cells_take_inputs_in_the_right_order() {
            let mut reactor = Reactor::new();
            let one = reactor.create_input(1);
            let two = reactor.create_input(2);
            let output = reactor
                  .create_compute(&[CellId::Input(one), CellId::Input(two)], |v| {
                        v[0] + v[1] * 10
                  })
                  .unwrap();
            assert_eq!(reactor.value(CellId::Compute(output)), Some(21));
      }

      #[test]
      fn error_creating_compute_cell_if_input_doesnt_exist() {
            let mut dummy_reactor = Reactor::new();
            let input = dummy_reactor.create_input(1);
            assert_eq!(
                  Reactor::new().create_compute(&[CellId::Input(input)], |_| 0),
                  Err(CellId::Input(input))
            );
      }

      #[test]
      fn do_not_break_cell_if_creating_compute_cell_with_valid_and_invalid_input() {
            let mut dummy_reactor = Reactor::new();
            let _ = dummy_reactor.create_input(1);
            let dummy_cell = dummy_reactor.create_input(2);
            let mut reactor = Reactor::new();
            let input = reactor.create_input(1);
            assert_eq!(
                  reactor.create_compute(
                        &[
                              CellId::Input(input),
                              CellId::Input(dummy_cell)
                        ],
                        |_| 0
                  ),
                  Err(CellId::Input(dummy_cell))
            );
            assert!(reactor.set_value(input, 5));
            assert_eq!(reactor.value(CellId::Input(input)), Some(5));
      }

      #[test]
      fn compute_cells_update_value_when_dependencies_are_changed() {
            let mut reactor = Reactor::new();
            let input = reactor.create_input(1);
            let output = reactor
                  .create_compute(&[CellId::Input(input)], |v| v[0] + 1)
                  .unwrap();
            assert_eq!(reactor.value(CellId::Compute(output)), Some(2));
            assert!(reactor.set_value(input, 3));
            assert_eq!(reactor.value(CellId::Compute(output)), Some(4));
      }

      #[test]
      fn compute_cells_can_depend_on_other_compute_cells() {
            let mut reactor = Reactor::new();
            let input = reactor.create_input(1);
            let times_two = reactor
                  .create_compute(&[CellId::Input(input)], |v| v[0] * 2)
                  .unwrap();
            let times_thirty = reactor
                  .create_compute(&[CellId::Input(input)], |v| v[0] * 30)
                  .unwrap();
            let output = reactor
                  .create_compute(
                        &[
                              CellId::Compute(times_two),
                              CellId::Compute(times_thirty),
                        ],
                        |v| v[0] + v[1],
                  )
                  .unwrap();
            assert_eq!(reactor.value(CellId::Compute(output)), Some(32));
            assert!(reactor.set_value(input, 3));
            assert_eq!(reactor.value(CellId::Compute(output)), Some(96));
      }

      /// A CallbackRecorder helps tests whether callbacks get called correctly.
      /// You'll see it used in tests that deal with callbacks.
      /// The names should be descriptive enough so that the tests make sense,
      /// so it's not necessary to fully understand the implementation,
      /// though you are welcome to.
      struct CallbackRecorder {
            // Note that this `Cell` is https://doc.rust-lang.org/std/cell/
            // a mechanism to allow internal mutability,
            // distinct from the cells (input cells, compute cells) in the reactor
            value: std::cell::Cell<Option<i32>>,
      }

      impl CallbackRecorder {
            fn new() -> Self {
                  CallbackRecorder {
                        value: std::cell::Cell::new(None),
                  }
            }

            fn expect_to_have_been_called_with(&self, v: i32) {
                  assert_ne!(
                        self.value.get(),
                        None,
                        "Callback was not called, but should have been"
                  );
                  assert_eq!(
                        self.value.replace(None),
                        Some(v),
                        "Callback was called with incorrect value"
                  );
            }

            fn expect_not_to_have_been_called(&self) {
                  assert_eq!(
                        self.value.get(),
                        None,
                        "Callback was called, but should not have been"
                  );
            }

            fn callback_called(&self, v: i32) {
                  assert_eq!(
                        self.value.replace(Some(v)),
                        None,
                        "Callback was called too many times; can't be called with {}",
                        v
                  );
            }
      }

      #[test]
      #[ignore]
      fn compute_cells_fire_callbacks() {
            let cb = CallbackRecorder::new();
            let mut reactor = C::new(Reactor::new());
            let input = reactor.get_mut().create_input(1);
            let output = reactor
                  .get_mut()
                  .create_compute(&[CellId::Input(input)], |v| v[0] + 1)
                  .unwrap();
            assert!(reactor
                  .get_mut()
                  .add_callback(output, |v| {
                        cb.callback_called(v);
                        assert!(reactor.get_mut().set_value(input, 3));
                        cb.expect_to_have_been_called_with(4);
                  })
                  .is_some());
      }

      #[test]
      #[ignore]
      fn error_adding_callback_to_nonexistent_cell() {
            let mut dummy_reactor = Reactor::new();
            let input = dummy_reactor.create_input(1);
            let output = dummy_reactor
                  .create_compute(&[CellId::Input(input)], |_| 0)
                  .unwrap();
            assert_eq!(
                  Reactor::new().add_callback(output, |_: u32| { println!("hi") }),
                  None
            );
      }

      #[test]
      #[ignore]
      fn error_removing_callback_from_nonexisting_cell() {
            let mut dummy_reactor = Reactor::new();
            let dummy_input = dummy_reactor.create_input(1);
            let _ = dummy_reactor
                  .create_compute(&[CellId::Input(dummy_input)], |_| 0)
                  .unwrap();
            let dummy_output = dummy_reactor
                  .create_compute(&[CellId::Input(dummy_input)], |_| 0)
                  .unwrap();
            let mut reactor = Reactor::new();
            let input = reactor.create_input(1);
            let output = reactor
                  .create_compute(&[CellId::Input(input)], |_| 0)
                  .unwrap();
            let callback = reactor.add_callback(output, |_| ()).unwrap();
            assert_eq!(
                  reactor.remove_callback(dummy_output, callback),
                  Err(RemoveCallbackError::NonexistentCell)
            );
      }

      #[test]
      #[ignore]
      fn callbacks_only_fire_on_change() {
            let cb = CallbackRecorder::new();
            let mut reactor = Reactor::new();
            let input = reactor.create_input(1);
            let output = reactor
                  .create_compute(
                        &[CellId::Input(input)],
                        |v| {
                              if v[0] < 3 {
                                    111
                              } else {
                                    222
                              }
                        },
                  )
                  .unwrap();
            assert!(reactor
                  .add_callback(output, |v| cb.callback_called(v))
                  .is_some());
            assert!(reactor.set_value(input, 2));
            cb.expect_not_to_have_been_called();
            assert!(reactor.set_value(input, 4));
            cb.expect_to_have_been_called_with(222);
      }

      #[test]
      #[ignore]
      fn callbacks_can_be_called_multiple_times() {
            let cb = CallbackRecorder::new();
            let mut reactor = Reactor::new();
            let input = reactor.create_input(1);
            let output = reactor
                  .create_compute(&[CellId::Input(input)], |v| v[0] + 1)
                  .unwrap();
            assert!(reactor
                  .add_callback(output, |v| cb.callback_called(v))
                  .is_some());
            assert!(reactor.set_value(input, 2));
            cb.expect_to_have_been_called_with(3);
            assert!(reactor.set_value(input, 3));
            cb.expect_to_have_been_called_with(4);
      }

      #[test]
      #[ignore]
      fn callbacks_can_be_called_from_multiple_cells() {
            let cb1 = CallbackRecorder::new();
            let cb2 = CallbackRecorder::new();
            let mut reactor = Reactor::new();
            let input = reactor.create_input(1);
            let plus_one = reactor
                  .create_compute(&[CellId::Input(input)], |v| v[0] + 1)
                  .unwrap();
            let minus_one = reactor
                  .create_compute(&[CellId::Input(input)], |v| v[0] - 1)
                  .unwrap();
            assert!(reactor
                  .add_callback(plus_one, |v| cb1.callback_called(v))
                  .is_some());
            assert!(reactor
                  .add_callback(minus_one, |v| cb2.callback_called(v))
                  .is_some());
            assert!(reactor.set_value(input, 10));
            cb1.expect_to_have_been_called_with(11);
            cb2.expect_to_have_been_called_with(9);
      }

      #[test]
      #[ignore]
      fn callbacks_can_be_added_and_removed() {
            let cb1 = CallbackRecorder::new();
            let cb2 = CallbackRecorder::new();
            let cb3 = CallbackRecorder::new();
            let mut reactor = Reactor::new();
            let input = reactor.create_input(11);
            let output = reactor
                  .create_compute(&[CellId::Input(input)], |v| v[0] + 1)
                  .unwrap();
            let callback = reactor
                  .add_callback(output, |v| cb1.callback_called(v))
                  .unwrap();
            assert!(reactor
                  .add_callback(output, |v| cb2.callback_called(v))
                  .is_some());
            assert!(reactor.set_value(input, 31));
            cb1.expect_to_have_been_called_with(32);
            cb2.expect_to_have_been_called_with(32);
            assert!(reactor.remove_callback(output, callback).is_ok());
            assert!(reactor
                  .add_callback(output, |v| cb3.callback_called(v))
                  .is_some());
            assert!(reactor.set_value(input, 41));
            cb1.expect_not_to_have_been_called();
            cb2.expect_to_have_been_called_with(42);
            cb3.expect_to_have_been_called_with(42);
      }

      #[test]
      #[ignore]
      fn removing_a_callback_multiple_times_doesnt_interfere_with_other_callbacks() {
            let cb1 = CallbackRecorder::new();
            let cb2 = CallbackRecorder::new();
            let mut reactor = Reactor::new();
            let input = reactor.create_input(1);
            let output = reactor
                  .create_compute(&[CellId::Input(input)], |v| v[0] + 1)
                  .unwrap();
            let callback = reactor
                  .add_callback(output, |v| cb1.callback_called(v))
                  .unwrap();
            assert!(reactor
                  .add_callback(output, |v| cb2.callback_called(v))
                  .is_some());
            // We want the first remove to be Ok, but the others should be errors.
            assert!(reactor.remove_callback(output, callback).is_ok());
            for _ in 1..5 {
                  assert_eq!(
                        reactor.remove_callback(output, callback),
                        Err(RemoveCallbackError::NonexistentCallback)
                  );
            }
            assert!(reactor.set_value(input, 2));
            cb1.expect_not_to_have_been_called();
            cb2.expect_to_have_been_called_with(3);
      }

      #[test]
      #[ignore]
      fn callbacks_should_only_be_called_once_even_if_multiple_dependencies_change() {
            let cb = CallbackRecorder::new();
            let mut reactor = Reactor::new();
            let input = reactor.create_input(1);
            let plus_one = reactor
                  .create_compute(&[CellId::Input(input)], |v| v[0] + 1)
                  .unwrap();
            let minus_one1 = reactor
                  .create_compute(&[CellId::Input(input)], |v| v[0] - 1)
                  .unwrap();
            let minus_one2 = reactor
                  .create_compute(&[CellId::Compute(minus_one1)], |v| v[0] - 1)
                  .unwrap();
            let output = reactor
                  .create_compute(
                        &[
                              CellId::Compute(plus_one),
                              CellId::Compute(minus_one2),
                        ],
                        |v| v[0] * v[1],
                  )
                  .unwrap();
            assert!(reactor
                  .add_callback(output, |v| cb.callback_called(v))
                  .is_some());
            assert!(reactor.set_value(input, 4));
            cb.expect_to_have_been_called_with(10);
      }

      #[test]
      #[ignore]
      fn callbacks_should_not_be_called_if_dependencies_change_but_output_value_doesnt_change() {
            let cb = CallbackRecorder::new();
            let mut reactor = Reactor::new();
            let input = reactor.create_input(1);
            let plus_one = reactor
                  .create_compute(&[CellId::Input(input)], |v| v[0] + 1)
                  .unwrap();
            let minus_one = reactor
                  .create_compute(&[CellId::Input(input)], |v| v[0] - 1)
                  .unwrap();
            let always_two = reactor
                  .create_compute(
                        &[
                              CellId::Compute(plus_one),
                              CellId::Compute(minus_one),
                        ],
                        |v| v[0] - v[1],
                  )
                  .unwrap();
            assert!(reactor
                  .add_callback(always_two, |v| cb.callback_called(v))
                  .is_some());
            for i in 2..5 {
                  assert!(reactor.set_value(input, i));
                  cb.expect_not_to_have_been_called();
            }
      }

      #[test]
      #[ignore]
      fn test_adder_with_boolean_values() {
            // This is a digital logic circuit called an adder:
            // https://en.wikipedia.org/wiki/Adder_(electronics)
            let mut reactor = Reactor::new();
            let a = reactor.create_input(false);
            let b = reactor.create_input(false);
            let carry_in = reactor.create_input(false);
            let a_xor_b = reactor
                  .create_compute(&[CellId::Input(a), CellId::Input(b)], |v| v[0] ^ v[1])
                  .unwrap();
            let sum = reactor
                  .create_compute(
                        &[
                              CellId::Compute(a_xor_b),
                              CellId::Input(carry_in),
                        ],
                        |v| v[0] ^ v[1],
                  )
                  .unwrap();
            let a_xor_b_and_cin = reactor
                  .create_compute(
                        &[
                              CellId::Compute(a_xor_b),
                              CellId::Input(carry_in),
                        ],
                        |v| v[0] && v[1],
                  )
                  .unwrap();
            let a_and_b = reactor
                  .create_compute(&[CellId::Input(a), CellId::Input(b)], |v| v[0] && v[1])
                  .unwrap();
            let carry_out = reactor
                  .create_compute(
                        &[
                              CellId::Compute(a_xor_b_and_cin),
                              CellId::Compute(a_and_b),
                        ],
                        |v| v[0] || v[1],
                  )
                  .unwrap();
            let tests = &[
                  (false, false, false, false, false),
                  (false, false, true, false, true),
                  (false, true, false, false, true),
                  (false, true, true, true, false),
                  (true, false, false, false, true),
                  (true, false, true, true, false),
                  (true, true, false, true, false),
                  (true, true, true, true, true),
            ];
            for &(aval, bval, cinval, expected_cout, expected_sum) in tests {
                  assert!(reactor.set_value(a, aval));
                  assert!(reactor.set_value(b, bval));
                  assert!(reactor.set_value(carry_in, cinval));
                  assert_eq!(reactor.value(CellId::Compute(sum)), Some(expected_sum));
                  assert_eq!(
                        reactor.value(CellId::Compute(carry_out)),
                        Some(expected_cout)
                  );
            }
      }
}
