#[allow(dead_code)]
pub mod graph {
    pub use std::collections::HashMap;
    pub type Attrs = [(&'static str, &'static str)];

    pub mod graph_items {
        pub use super::{Attrs, HashMap};

        pub mod edge {
            use super::{Attrs, HashMap};

            #[derive(Debug, PartialEq, Eq, Clone)]
            pub struct Edge {
                pub attrs: HashMap<&'static str, &'static str>,
            }

            impl Edge {
                pub fn new(a: &'static str, b: &'static str) -> Self {
                    Edge { attrs: maplit::hashmap! {
                               a => b
                           }, }
                }

                pub fn with_attrs(mut self, attrs: &Attrs) -> Self {
                    for &(attr, value) in attrs.iter() {
                        self.attrs.insert(attr, value);
                    }
                    self
                }

                pub fn attr(&self, a: &'static str) -> Option<&str> {
                    // All works:
                    // self.attrs.get(&a).and_then(|a| Some(*a))
                    // self.attrs.get(&a).map(|a| *a)
                    self.attrs.get(&a).copied()
                }
            }
        }

        pub mod node {

            use super::{Attrs, HashMap};

            #[derive(Debug, PartialEq, Eq, Clone)]
            pub struct Node {
                pub name:  &'static str,
                pub attrs: HashMap<&'static str, &'static str>,
            }

            impl Node {
                pub fn new(name: &'static str) -> Self {
                    let attrs = HashMap::<&'static str, &'static str>::new();
                    Node { name, attrs }
                }

                pub fn with_attrs(mut self, attrs: &Attrs) -> Self {
                    self.attrs.extend(attrs.iter().cloned());
                    self
                }

                pub fn attr(&self, a: &'static str) -> Option<&str> {
                    self.attrs.get(&a).copied()
                }
            }
        }
    }

    use graph_items::{edge::Edge, node::Node};

    #[derive(Debug, PartialEq, Eq)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            let g = Graph { edges: Vec::<Edge>::new(),
                            nodes: Vec::<Node>::new(),
                            attrs: HashMap::<String, String>::new(), };
            println!("{}\t{}\t{}",
                     g.nodes.is_empty(),
                     g.edges.is_empty(),
                     g.attrs.is_empty());
            g
        }

        pub fn with_attrs(mut self, attrs: &Attrs) -> Self {
            for &(attr, value) in attrs.iter() {
                self.attrs
                    .insert(attr.to_string(), value.to_string());
            }
            self
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend(nodes.iter().cloned());
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend(edges.iter().cloned());
            self
        }

        pub fn node(&self, name: &'static str) -> Option<&Node> {
            let mut node: Option<&Node> = None;
            for n in self.nodes.iter() {
                if n.name.eq(name) {
                    node = Some(n);
                    break;
                }
            }
            node
        }
    }
}

#[cfg(test)]

mod tests {
    use super::graph::{graph_items::{edge::Edge, node::Node},
                       Graph};
    use maplit::hashmap;

    #[test]
    fn test_empty_graph() {
        let graph = Graph::new();
        assert!(graph.nodes.is_empty());
        assert!(graph.edges.is_empty());
        assert!(graph.attrs.is_empty());
    }

    #[test]
    fn test_graph_with_one_node() {
        let nodes = vec![Node::new("a")];
        let graph = Graph::new().with_nodes(&nodes);
        assert!(graph.edges.is_empty());
        assert!(graph.attrs.is_empty());
        assert_eq!(graph.nodes, vec![Node::new("a")]);
    }

    #[test]
    fn test_graph_with_one_node_with_keywords() {
        let nodes = vec![Node::new("a").with_attrs(&[("color", "green")])];
        let graph = Graph::new().with_nodes(&nodes);
        assert!(graph.edges.is_empty());
        assert!(graph.attrs.is_empty());
        assert_eq!(graph.nodes, vec![Node::new("a").with_attrs(&[("color",
                                                                  "green")])]);
    }

    #[test]
    fn test_graph_with_one_edge() {
        let edges = vec![Edge::new("a", "b")];
        let graph = Graph::new().with_edges(&edges);
        assert!(graph.nodes.is_empty());
        assert!(graph.attrs.is_empty());
        assert_eq!(graph.edges, vec![Edge::new("a", "b")]);
    }

    #[test]
    fn test_graph_with_one_edge_with_keywords() {
        let edges = vec![Edge::new("a", "b").with_attrs(&[("color", "blue")])];
        let graph = Graph::new().with_edges(&edges);
        assert!(graph.nodes.is_empty());
        assert!(graph.attrs.is_empty());
        assert_eq!(graph.edges,
                   vec![Edge::new("a", "b").with_attrs(&[("color", "blue")])]);
    }

    #[test]
    fn test_graph_with_one_attribute() {
        let graph = Graph::new().with_attrs(&[("foo", "1")]);
        let expected_attrs = hashmap! {
            "foo".to_string() => "1".to_string(),
        };
        assert!(graph.nodes.is_empty());
        assert!(graph.edges.is_empty());
        assert_eq!(graph.attrs, expected_attrs);
    }

    #[test]
    fn test_graph_with_attributes() {
        let nodes = vec![Node::new("a").with_attrs(&[("color", "green")]),
                         Node::new("c"),
                         Node::new("b").with_attrs(&[("label", "Beta!")]),];
        let edges = vec![Edge::new("b", "c"),
                         Edge::new("a", "b").with_attrs(&[("color", "blue")]),];
        let attrs = vec![("foo", "1"),
                         ("title", "Testing Attrs"),
                         ("bar", "true")];
        let expected_attrs = hashmap! {
            "foo".to_string() => "1".to_string(),
            "title".to_string() => "Testing Attrs".to_string(),
            "bar".to_string() => "true".to_string(),
        };
        let graph = Graph::new().with_nodes(&nodes)
                                .with_edges(&edges)
                                .with_attrs(&attrs);
        assert_eq!(graph.nodes, vec![Node::new("a").with_attrs(&[("color",
                                                                  "green")]),
                                     Node::new("c"),
                                     Node::new("b").with_attrs(&[("label",
                                                                  "Beta!")]),]);
        assert_eq!(graph.edges, vec![
            Edge::new("b", "c"),
            Edge::new("a", "b").with_attrs(&[("color", "blue")]),
        ]);
        assert_eq!(graph.attrs, expected_attrs);
    }

    #[test]
    fn test_edges_store_attributes() {
        let nodes = vec![Node::new("a").with_attrs(&[("color", "green")]),
                         Node::new("c"),
                         Node::new("b").with_attrs(&[("label", "Beta!")]),];
        let edges = vec![Edge::new("b", "c"),
                         Edge::new("a", "b").with_attrs(&[("color", "blue"),
                                                          ("fill", "darkblue")]),];
        let attrs = vec![("foo", "1"),
                         ("title", "Testing Attrs"),
                         ("bar", "true")];
        let graph = Graph::new().with_nodes(&nodes)
                                .with_edges(&edges)
                                .with_attrs(&attrs);
        assert_eq!(graph.edges, vec![
            Edge::new("b", "c"),
            Edge::new("a", "b").with_attrs(&[("color", "blue"), ("fill", "darkblue")]),
        ]);
        assert_eq!(graph.edges[1].attr("color"), Some("blue"));
        assert_eq!(graph.edges[1].attr("fill"), Some("darkblue"));
        assert_eq!(graph.edges[1].attr("foo"), None);
        assert_eq!(graph.edges[0].attr("color"), None);
        assert_eq!(graph.edges[0].attr("fill"), None);
        assert_eq!(graph.edges[0].attr("foo"), None);
    }

    #[test]
    fn test_graph_nodes_store_attributes() {
        let attributes = [("foo", "bar"),
                          ("bat", "baz"),
                          ("bim", "bef")];
        let graph = Graph::new().with_nodes(
        &["a", "b", "c"]
            .iter()
            .zip(attributes.iter())
            .map(|(name, &attr)| Node::new(name).with_attrs(&[attr]))
            .collect::<Vec<_>>(),
    );
        let a = graph.node("a")
                     .expect("node a must be stored");
        assert_eq!(a.attr("foo"), Some("bar"));
        assert_eq!(a.attr("bat"), None);
        assert_eq!(a.attr("bim"), None);
        let b = graph.node("b")
                     .expect("node b must be stored");
        assert_eq!(b.attr("foo"), None);
        assert_eq!(b.attr("bat"), Some("baz"));
        assert_eq!(b.attr("bim"), None);
        let c = graph.node("c")
                     .expect("node c must be stored");
        assert_eq!(c.attr("foo"), None);
        assert_eq!(c.attr("bat"), None);
        assert_eq!(c.attr("bim"), Some("bef"));
    }
}
