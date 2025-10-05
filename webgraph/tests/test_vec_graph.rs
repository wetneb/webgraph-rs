/*
 * SPDX-FileCopyrightText: 2023 Inria
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

#[cfg(feature = "serde")]
#[test]
fn test_serde() {
    use webgraph::graphs::vec_graph::LabeledVecGraph;
    let arcs = [(0, 1, 1), (0, 2, 2), (1, 2, 3)];

    let g = LabeledVecGraph::<usize>::from_arcs(arcs);
    let res = serde_json::to_string(&g).unwrap();
    let p: LabeledVecGraph<usize> = serde_json::from_str(&res).unwrap();
    assert_eq!(g, p);
}
