#![cfg_attr(feature = "test", feature(test))]

#[cfg(feature = "test")]
extern crate test;
extern crate petgraph;

use petgraph::{
    Graph,
    Directed,
    Undirected,
    EdgeType,
};
use petgraph::graph::{
    NodeIndex,
};

/// Petersen A and B are isomorphic
///
/// http://www.dharwadker.org/tevet/isomorphism/
const PETERSEN_A: &'static str = "
 0 1 0 0 1 0 1 0 0 0 
 1 0 1 0 0 0 0 1 0 0 
 0 1 0 1 0 0 0 0 1 0 
 0 0 1 0 1 0 0 0 0 1 
 1 0 0 1 0 1 0 0 0 0 
 0 0 0 0 1 0 0 1 1 0 
 1 0 0 0 0 0 0 0 1 1 
 0 1 0 0 0 1 0 0 0 1 
 0 0 1 0 0 1 1 0 0 0 
 0 0 0 1 0 0 1 1 0 0
";

const PETERSEN_B: &'static str = "
 0 0 0 1 0 1 0 0 0 1 
 0 0 0 1 1 0 1 0 0 0 
 0 0 0 0 0 0 1 1 0 1 
 1 1 0 0 0 0 0 1 0 0
 0 1 0 0 0 0 0 0 1 1 
 1 0 0 0 0 0 1 0 1 0 
 0 1 1 0 0 1 0 0 0 0 
 0 0 1 1 0 0 0 0 1 0 
 0 0 0 0 1 1 0 1 0 0 
 1 0 1 0 1 0 0 0 0 0
";

/// An almost full set, isomorphic
const FULL_A: &'static str = "
 1 1 1 1 1 1 1 1 1 1 
 1 1 1 1 1 1 1 1 1 1 
 1 1 1 1 1 1 1 1 1 1 
 1 1 1 1 1 1 1 1 1 1 
 1 1 1 1 1 1 1 1 1 1 
 1 1 1 1 1 1 1 1 1 1 
 1 1 1 1 1 1 1 1 1 1 
 1 1 1 1 1 1 1 1 1 1 
 1 1 1 1 0 1 1 1 0 1 
 1 1 1 1 1 1 1 1 1 1
";

const FULL_B: &'static str = "
 1 1 1 1 1 1 1 1 1 1 
 1 1 1 1 1 1 1 1 1 1 
 1 1 0 1 1 1 0 1 1 1 
 1 1 1 1 1 1 1 1 1 1
 1 1 1 1 1 1 1 1 1 1 
 1 1 1 1 1 1 1 1 1 1 
 1 1 1 1 1 1 1 1 1 1 
 1 1 1 1 1 1 1 1 1 1 
 1 1 1 1 1 1 1 1 1 1 
 1 1 1 1 1 1 1 1 1 1
";

/// Praust A and B are not isomorphic
const PRAUST_A: &'static str = "
 0 1 1 1 1 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 
 1 0 1 1 0 1 0 0 0 1 0 0 0 0 0 0 0 0 0 0 
 1 1 0 1 0 0 1 0 0 0 1 0 0 0 0 0 0 0 0 0 
 1 1 1 0 0 0 0 1 0 0 0 1 0 0 0 0 0 0 0 0 
 1 0 0 0 0 1 1 1 0 0 0 0 1 0 0 0 0 0 0 0 
 0 1 0 0 1 0 1 1 0 0 0 0 0 1 0 0 0 0 0 0 
 0 0 1 0 1 1 0 1 0 0 0 0 0 0 1 0 0 0 0 0 
 0 0 0 1 1 1 1 0 0 0 0 0 0 0 0 1 0 0 0 0 
 1 0 0 0 0 0 0 0 0 1 1 1 0 0 0 0 1 0 0 0 
 0 1 0 0 0 0 0 0 1 0 1 1 0 0 0 0 0 1 0 0 
 0 0 1 0 0 0 0 0 1 1 0 1 0 0 0 0 0 0 1 0 
 0 0 0 1 0 0 0 0 1 1 1 0 0 0 0 0 0 0 0 1 
 0 0 0 0 1 0 0 0 0 0 0 0 0 1 1 1 0 1 0 0 
 0 0 0 0 0 1 0 0 0 0 0 0 1 0 1 1 1 0 0 0 
 0 0 0 0 0 0 1 0 0 0 0 0 1 1 0 1 0 0 0 1 
 0 0 0 0 0 0 0 1 0 0 0 0 1 1 1 0 0 0 1 0 
 0 0 0 0 0 0 0 0 1 0 0 0 0 1 0 0 0 1 1 1 
 0 0 0 0 0 0 0 0 0 1 0 0 1 0 0 0 1 0 1 1 
 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 1 1 1 0 1 
 0 0 0 0 0 0 0 0 0 0 0 1 0 0 1 0 1 1 1 0
";

const PRAUST_B: &'static str = "
 0 1 1 1 1 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 
 1 0 1 1 0 1 0 0 0 1 0 0 0 0 0 0 0 0 0 0 
 1 1 0 1 0 0 1 0 0 0 1 0 0 0 0 0 0 0 0 0 
 1 1 1 0 0 0 0 1 0 0 0 1 0 0 0 0 0 0 0 0 
 1 0 0 0 0 1 1 1 0 0 0 0 1 0 0 0 0 0 0 0 
 0 1 0 0 1 0 1 1 0 0 0 0 0 0 0 0 0 0 0 1 
 0 0 1 0 1 1 0 1 0 0 0 0 0 0 1 0 0 0 0 0 
 0 0 0 1 1 1 1 0 0 0 0 0 0 0 0 0 0 1 0 0 
 1 0 0 0 0 0 0 0 0 1 1 1 0 0 0 0 1 0 0 0
 0 1 0 0 0 0 0 0 1 0 1 1 0 1 0 0 0 0 0 0 
 0 0 1 0 0 0 0 0 1 1 0 1 0 0 0 0 0 0 1 0 
 0 0 0 1 0 0 0 0 1 1 1 0 0 0 0 1 0 0 0 0 
 0 0 0 0 1 0 0 0 0 0 0 0 0 1 1 0 0 1 0 1 
 0 0 0 0 0 0 0 0 0 1 0 0 1 0 0 1 1 0 1 0 
 0 0 0 0 0 0 1 0 0 0 0 0 1 0 0 1 0 1 0 1 
 0 0 0 0 0 0 0 0 0 0 0 1 0 1 1 0 1 0 1 0 
 0 0 0 0 0 0 0 0 1 0 0 0 0 1 0 1 0 1 1 0 
 0 0 0 0 0 0 0 1 0 0 0 0 1 0 1 0 1 0 0 1 
 0 0 0 0 0 0 0 0 0 0 1 0 0 1 0 1 1 0 0 1 
 0 0 0 0 0 1 0 0 0 0 0 0 1 0 1 0 0 1 1 0 
";

const G1U: &'static str = "
0 1 1 0 1
1 0 1 0 0
1 1 0 0 0
0 0 0 0 0
1 0 0 0 0
";

const G2U: &'static str = "
0 1 0 1 0
1 0 0 1 1
0 0 0 0 0
1 1 0 0 0
0 1 0 0 0
";

const G4U: &'static str = "
0 1 1 0 1
1 0 0 1 0
1 0 0 0 0
0 1 0 0 0
1 0 0 0 0
";

const G1D: &'static str = "
0 1 1 0 1
0 0 1 0 0
0 0 0 0 0
0 0 0 0 0
0 0 0 0 0
";

const G4D: &'static str = "
0 1 1 0 1
0 0 0 1 0
0 0 0 0 0
0 0 0 0 0
0 0 0 0 0
";

// G8 1,2 are not iso
const G8_1: &'static str = "
0 1 1 0 0 1 1 1
1 0 1 0 1 0 1 1
1 1 0 1 0 0 1 1
0 0 1 0 1 1 1 1
0 1 0 1 0 1 1 1
1 0 0 1 1 0 1 1
1 1 1 1 1 1 0 1
1 1 1 1 1 1 1 0
";

const G8_2: &'static str = "
0 1 0 1 0 1 1 1
1 0 1 0 1 0 1 1
0 1 0 1 0 1 1 1
1 0 1 0 1 0 1 1
0 1 0 1 0 1 1 1
1 0 1 0 1 0 1 1
1 1 1 1 1 1 0 1
1 1 1 1 1 1 1 0
";

// G3 1,2 are not iso
const G3_1: &'static str = "
0 1 0
1 0 1
0 1 0
";
const G3_2: &'static str = "
0 1 1
1 0 1
1 1 0
";

// Non-isomorphic due to selfloop difference
const S1: &'static str = "
1 1 1
1 0 1
1 0 0
";
const S2: &'static str = "
1 1 1
0 1 1
1 0 0
";

/// Parse a text adjacency matrix format into a directed graph
fn parse_graph<Ty: EdgeType = Directed>(s: &str) -> Graph<(), (), Ty>
{
    let mut gr = Graph::with_capacity(0, 0);
    let s = s.trim();
    let lines = s.lines().filter(|l| !l.is_empty());
    for (row, line) in lines.enumerate() {
        for (col, word) in line.split(' ')
                                .filter(|s| s.len() > 0)
                                .enumerate()
        {
            let has_edge = word.parse::<i32>().unwrap();
            assert!(has_edge == 0 || has_edge == 1);
            if has_edge == 0 {
                continue;
            }
            while col >= gr.node_count() || row >= gr.node_count() {
                gr.add_node(());
            }
            gr.update_edge(NodeIndex::new(row), NodeIndex::new(col), ());
        }
    }
    gr
}

fn str_to_graph(s: &str) -> Graph<(), (), Undirected> {
    parse_graph(s)
}

fn str_to_digraph(s: &str) -> Graph<(), (), Directed> {
    parse_graph(s)
}

/*
fn graph_to_ad_matrix<N, E, Ty: EdgeType>(g: &Graph<N,E,Ty>)
{
    let n = g.node_count();
    for i in (0..n) {
        for j in (0..n) {
            let ix = NodeIndex::new(i);
            let jx = NodeIndex::new(j);
            let out = match g.find_edge(ix, jx) {
                None => "0",
                Some(_) => "1",
            };
            print!("{} ", out);
        }
        println!("");
    }
}
*/

#[test]
fn petersen_iso()
{
    // The correct isomorphism is
    // 0 => 0, 1 => 3, 2 => 1, 3 => 4, 5 => 2, 6 => 5, 7 => 7, 8 => 6, 9 => 8, 4 => 9
    let peta = str_to_digraph(PETERSEN_A);
    let petb = str_to_digraph(PETERSEN_B);
    /*
    println!("{:?}", peta);
    graph_to_ad_matrix(&peta);
    println!("");
    graph_to_ad_matrix(&petb);
    */

    assert!(petgraph::algo::is_isomorphic(&peta, &petb));
}

#[test]
fn petersen_undir_iso()
{
    // The correct isomorphism is
    // 0 => 0, 1 => 3, 2 => 1, 3 => 4, 5 => 2, 6 => 5, 7 => 7, 8 => 6, 9 => 8, 4 => 9
    let peta = str_to_digraph(PETERSEN_A);
    let petb = str_to_digraph(PETERSEN_B);

    assert!(petgraph::algo::is_isomorphic(&peta, &petb));
}

#[test]
fn full_iso()
{
    let a = str_to_graph(FULL_A);
    let b = str_to_graph(FULL_B);

    assert!(petgraph::algo::is_isomorphic(&a, &b));
}

#[test]
fn praust_dir_no_iso()
{
    let a = str_to_digraph(PRAUST_A);
    let b = str_to_digraph(PRAUST_B);

    assert!(!petgraph::algo::is_isomorphic(&a, &b));
}

#[test]
fn praust_undir_no_iso()
{
    let a = str_to_graph(PRAUST_A);
    let b = str_to_graph(PRAUST_B);

    assert!(!petgraph::algo::is_isomorphic(&a, &b));
}

#[cfg(feature = "test")]
#[bench]
fn petersen_iso_bench(bench: &mut test::Bencher)
{
    let a = str_to_digraph(PETERSEN_A);
    let b = str_to_digraph(PETERSEN_B);

    bench.iter(|| petgraph::algo::is_isomorphic(&a, &b));
}

#[cfg(feature = "test")]
#[bench]
fn petersen_undir_iso_bench(bench: &mut test::Bencher)
{
    let a = str_to_graph(PETERSEN_A);
    let b = str_to_graph(PETERSEN_B);

    bench.iter(|| petgraph::algo::is_isomorphic(&a, &b));
}

#[cfg(feature = "test")]
#[bench]
fn full_iso_bench(bench: &mut test::Bencher)
{
    let a = str_to_graph(FULL_A);
    let b = str_to_graph(FULL_B);

    bench.iter(|| petgraph::algo::is_isomorphic(&a, &b));
}

#[cfg(feature = "test")]
#[bench]
fn praust_dir_no_iso_bench(bench: &mut test::Bencher)
{
    let a = str_to_digraph(PRAUST_A);
    let b = str_to_digraph(PRAUST_B);

    bench.iter(|| petgraph::algo::is_isomorphic(&a, &b));
}

#[cfg(feature = "test")]
#[bench]
fn praust_undir_no_iso_bench(bench: &mut test::Bencher)
{
    let a = str_to_graph(PRAUST_A);
    let b = str_to_graph(PRAUST_B);

    bench.iter(|| petgraph::algo::is_isomorphic(&a, &b));
}

#[test]
fn coxeter_di_iso()
{
    // The correct isomorphism is
    let a = str_to_digraph(COXETER_A);
    let b = str_to_digraph(COXETER_B);
    assert!(petgraph::algo::is_isomorphic(&a, &b));
}

#[test]
fn coxeter_undi_iso()
{
    // The correct isomorphism is
    let a = str_to_graph(COXETER_A);
    let b = str_to_graph(COXETER_B);
    assert!(petgraph::algo::is_isomorphic(&a, &b));
}

#[test]
fn g14_dir_not_iso()
{
    let a = str_to_digraph(G1D);
    let b = str_to_digraph(G4D);
    assert!(!petgraph::algo::is_isomorphic(&a, &b));
}

#[test]
fn g14_undir_not_iso()
{
    let a = str_to_digraph(G1U);
    let b = str_to_digraph(G4U);
    assert!(!petgraph::algo::is_isomorphic(&a, &b));
}

#[test]
fn g12_undir_iso()
{
    let a = str_to_digraph(G1U);
    let b = str_to_digraph(G2U);
    assert!(petgraph::algo::is_isomorphic(&a, &b));
}

#[test]
fn g3_not_iso()
{
    let a = str_to_digraph(G3_1);
    let b = str_to_digraph(G3_2);
    assert!(!petgraph::algo::is_isomorphic(&a, &b));
}

#[test]
fn g8_not_iso()
{
    let a = str_to_digraph(G8_1);
    let b = str_to_digraph(G8_2);
    assert_eq!(a.edge_count(), b.edge_count());
    assert_eq!(a.node_count(), b.node_count());
    assert!(!petgraph::algo::is_isomorphic(&a, &b));
}

#[test]
fn s12_not_iso()
{
    let a = str_to_digraph(S1);
    let b = str_to_digraph(S2);
    assert_eq!(a.edge_count(), b.edge_count());
    assert_eq!(a.node_count(), b.node_count());
    assert!(!petgraph::algo::is_isomorphic(&a, &b));
}

#[test]
fn iso1()
{
    let mut g0 = Graph::<_, ()>::new();
    let mut g1 = Graph::<_, ()>::new();
    assert!(petgraph::algo::is_isomorphic(&g0, &g1));

    // very simple cases
    let a0 = g0.add_node(0);
    let a1 = g1.add_node(0);
    assert!(petgraph::algo::is_isomorphic(&g0, &g1));
    let b0 = g0.add_node(1);
    let b1 = g1.add_node(1);
    assert!(petgraph::algo::is_isomorphic(&g0, &g1));
    let _ = g0.add_node(2);
    assert!(!petgraph::algo::is_isomorphic(&g0, &g1));
    let _ = g1.add_node(2);
    assert!(petgraph::algo::is_isomorphic(&g0, &g1));
    g0.add_edge(a0, b0, ());
    assert!(!petgraph::algo::is_isomorphic(&g0, &g1));
    g1.add_edge(a1, b1, ());
    assert!(petgraph::algo::is_isomorphic(&g0, &g1));
}

#[test]
fn iso2()
{
    let mut g0 = Graph::<_, ()>::new();
    let mut g1 = Graph::<_, ()>::new();

    let a0 = g0.add_node(0);
    let a1 = g1.add_node(0);
    let b0 = g0.add_node(1);
    let b1 = g1.add_node(1);
    let c0 = g0.add_node(2);
    let c1 = g1.add_node(2);
    g0.add_edge(a0, b0, ());
    g1.add_edge(c1, b1, ());
    assert!(petgraph::algo::is_isomorphic(&g0, &g1));
    // a -> b
    // a -> c
    // vs.
    // c -> b
    // c -> a
    g0.add_edge(a0, c0, ());
    g1.add_edge(c1, a1, ());
    assert!(petgraph::algo::is_isomorphic(&g0, &g1));

    // add
    // b -> c
    // vs
    // b -> a

    let _ = g0.add_edge(b0, c0, ());
    let _ = g1.add_edge(b1, a1, ());
    assert!(petgraph::algo::is_isomorphic(&g0, &g1));
    let d0 = g0.add_node(3);
    let d1 = g1.add_node(3);
    let e0 = g0.add_node(4);
    let e1 = g1.add_node(4);
    assert!(petgraph::algo::is_isomorphic(&g0, &g1));
    // add
    // b -> e -> d
    // vs
    // b -> d -> e
    g0.add_edge(b0, e0, ());
    g0.add_edge(e0, d0, ());
    g1.add_edge(b1, d1, ());
    g1.add_edge(d1, e1, ());
    assert!(petgraph::algo::is_isomorphic(&g0, &g1));
}


/// Isomorphic pair
const COXETER_A: &'static str = "
 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 1 
 1 0 1 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
 0 1 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 
 0 0 1 0 1 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
 0 0 0 1 0 1 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 
 0 0 0 0 1 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 
 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 
 0 0 0 0 0 0 1 0 1 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
 0 1 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 
 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 
 0 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0
 0 0 0 1 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 
 0 0 0 0 0 0 0 1 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 1 0 0 0 0 0 
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 0 0 0 0 1 
 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 0 0 0 0 
 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 0 0 0 
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 1 0 0 0 
 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 0 
 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 
 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 
 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 1 0 1 0 0 0 0 
 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 1 0 1 0 0 
 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 1 0 
 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 1 
 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 1 0 
";

const COXETER_B: &'static str = "
 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 1 0 0 0 0 0 0 0 0 0 1 0 0 
 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 1 0 0 0 0 0 
 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 1 0 0 1 0 0 0 0 0 0 
 0 0 0 0 0 0 1 0 0 1 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
 0 1 0 0 0 0 0 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 1 0 0 0 0 0 0 0 0 0 1 0 0 0 
 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 1 0 0 0 0 0 
 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 1 0 0 0 0 0 0 0 
 0 0 0 0 1 0 0 0 0 0 0 1 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
 0 0 0 1 0 0 0 0 0 0 0 0 1 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 0 
 0 0 0 0 0 1 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 
 0 0 0 0 0 0 0 0 0 1 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 
 1 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 
 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 1 0 0 0 0 0 0 
 0 0 0 0 0 0 0 0 1 1 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 
 0 0 0 0 0 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 
 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 1 
 0 0 0 0 0 0 0 1 0 0 0 0 0 1 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 
 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 1 
 0 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 
 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 0 0 0 0 
 0 0 0 0 0 0 0 1 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 
 0 0 1 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 
 0 1 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 
 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 1 0 0 0 0 0 1 0 0 0 
 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 1 0 
 1 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 1 0 0 1 
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 0 1 0
";
