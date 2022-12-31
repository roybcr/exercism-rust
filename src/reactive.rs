use std::collections::{HashMap, HashSet};

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputCellId;
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually
/// assignable, demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComputeCellId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
      Input(InputCellId),
      Compute(ComputeCellId),
}
struct InputCell<T> {
      value:        T,
      dependencies: Vec<ComputeCellId>,
}

struct ComputeCell<T> {
      dependencies: Vec<CellId>,
      compute_func: Box<dyn Fn(&[T]) -> T>,
      value:        T,
      callbacks:    Vec<CallbackId>,
}

struct Reactor<T> {
      input_cells:      HashMap<InputCellId, InputCell<T>>,
      compute_cells:    HashMap<ComputeCellId, ComputeCell<T>>,
      next_callback_id: CallbackId,
}

impl<T: Copy + PartialEq> Reactor<T> {
      pub fn new() -> Self {
            Self {
                  input_cells:      HashMap::new(),
                  compute_cells:    HashMap::new(),
                  next_callback_id: CallbackId(0),
            }
      }

      // Creates an input cell with the specified initial value, returning its ID.
      pub fn create_input(&mut self, initial: T) -> InputCellId {
            let id = InputCellId(self.input_cells.len() as u64);
            self.input_cells.insert(id, InputCell {
                  value:        initial,
                  dependencies: Vec::new(),
            });
            id
      }

      // Creates a compute cell with the specified dependencies and compute function.
      // The compute function is expected to take in its arguments in the same order
      // as specified in `dependencies`.
      // You do not need to reject compute functions that expect more arguments than
      // there are dependencies (how would you check for this, anyway?).
      //
      // If any dependency doesn't exist, returns an Err with that nonexistent
      // dependency. (If multiple dependencies do not exist, exactly which one is
      // returned is not defined and will not be tested)
      //
      // Notice that there is no way to *remove* a cell.
      // This means that you may assume, without checking, that if the dependencies
      // exist at creation time they will continue to exist as long as the Reactor
      // exists.

      pub fn create_compute<F: Fn(&[T]) -> T>(
            &mut self,
            dependencies: &[CellId],
            compute_func: F,
      ) -> Result<ComputeCellId, CellId> {
            let id = ComputeCellId(self.compute_cells.len() as u64);
            let dependencies: Result<Vec<_>, _> = dependencies
                  .iter()
                  .map(|dep| match dep {
                        CellId::Input(input_id) => {
                              if let Some(input_cell) = self.input_cells.get_mut(input_id) {
                                    input_cell.dependencies.push(id);
                                    Ok(dep.clone())
                              } else {
                                    Err(*dep)
                              }
                        },
                        CellId::Compute(compute_id) => Ok(dep.clone()),
                  })
                  .collect();
            let dependencies = dependencies?;

            // Compute the initial value of the cell
            let value = (compute_func)(&Self::eval_cell_ids(
                  &dependencies,
                  &self.input_cells,
                  &self.compute_cells,
            ));

            self.compute_cells.insert(id, ComputeCell {
                  dependencies,
                  compute_func: Box::new(compute_func),
                  value,
                  callbacks: Vec::new(),
            });

            Ok(id)
      }

      pub fn set_value(&mut self, id: InputCellId, value: T) {
            if let Some(input_cell) = self.input_cells.get_mut(&id) {
                  input_cell.value = value;
                  self.update_dependencies(id);
            }
      }

      fn update_dependencies(&mut self, id: InputCellId) {
            let mut to_update = Vec::new();
            let mut updated = HashSet::new();

            // Add all compute cells that depend on the input cell to the to_update list
            if let Some(input_cell) = self.input_cells.get(&id) {
                  to_update.extend(input_cell.dependencies.iter().cloned());
                  updated.insert(id);
            }

            // Iterate through the to_update list and update the values of the compute cells
            while let Some(compute_id) = to_update.pop() {
                  if let Some(compute_cell) = self.compute_cells.get_mut(&compute_id) {
                        let old_value = compute_cell.value;
                        compute_cell.value = (compute_cell.compute_func)(&Self::eval_cell_ids(
                              &compute_cell.dependencies,
                              &self.input_cells,
                              &self.compute_cells,
                        ));
                        if old_value != compute_cell.value {
                              for callback_id in &compute_cell.callbacks {
                                    self.callbacks.push((callback_id.clone(), compute_id));
                              }
                        }
                        for dep in &compute_cell.dependencies {
                              if !updated.contains(dep) {
                                    to_update.push(match dep {
                                          CellId::Input(input_id) => input_id,
                                          CellId::Compute(compute_id) => compute_id,
                                    });
                                    updated.insert(*dep);
                              }
                        }
                  }
            }
      }
}
