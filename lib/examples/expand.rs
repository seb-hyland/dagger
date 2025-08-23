#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use std::ops::Deref;
use dagger::dagger;
fn main() {
    let f32toi32 = |v: f32| -> Result<i32, &'static str> { Ok(v.floor() as i32) };
    let operation = {
        use dagger::prelude::*;
        Graph::new(
            || {
                let __private_cic = ProcessData::default();
                let __private_d = ProcessData::default();
                let __private_e = ProcessData::default();
                let __private_f = ProcessData::default();
                let __private_g = ProcessData::default();
                let __private_h = ProcessData::default();
                let __private_i = ProcessData::default();
                let __private_g_str_1 = ProcessData::default();
                let __private_g_str = ProcessData::default();
                let __private_g_str_array = ProcessData::default();
                let __private_out = ProcessData::default();
                let (__private_cic_tx, __private_cic_rx) = __private_cic.channel();
                let (__private_d_tx, __private_d_rx) = __private_d.channel();
                let (__private_e_tx, __private_e_rx) = __private_e.channel();
                let (__private_f_tx, __private_f_rx) = __private_f.channel();
                let (__private_g_tx, __private_g_rx) = __private_g.channel();
                let (__private_h_tx, __private_h_rx) = __private_h.channel();
                let (__private_i_tx, __private_i_rx) = __private_i.channel();
                let (__private_g_str_1_tx, __private_g_str_1_rx) = __private_g_str_1
                    .channel();
                let (__private_g_str_tx, __private_g_str_rx) = __private_g_str.channel();
                let (__private_g_str_array_tx, __private_g_str_array_rx) = __private_g_str_array
                    .channel();
                let (__private_out_tx, __private_out_rx) = __private_out.channel();
                ::std::thread::scope(|s| {
                    {
                        let node_name = "cic";
                        s.spawn(|| {
                            if true {
                                let () = ();
                                let process_result = sum(3, 5)
                                    .into_process_result(node_name);
                                __private_cic_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                __private_cic_tx.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "d";
                        s.spawn(|| {
                            if true {
                                let () = ();
                                let process_result = mult(3, 5)
                                    .into_process_result(node_name);
                                __private_d_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                __private_d_tx.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "e";
                        s.spawn(|| {
                            if true {
                                let () = ();
                                let process_result = div(0, 0)
                                    .into_process_result(node_name);
                                __private_e_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                __private_e_tx.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "f";
                        s.spawn(|| {
                            if true {
                                let () = ();
                                let process_result = div(0, 0)
                                    .into_process_result(node_name);
                                __private_f_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                __private_f_tx.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "g";
                        s.spawn(|| {
                            if true {
                                let () = ();
                                let process_result = div(3, 5)
                                    .into_process_result(node_name);
                                __private_g_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                __private_g_tx.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "h";
                        s.spawn(|| {
                            let g = __private_g_rx.wait();
                            if g.is_ok() {
                                let (g) = (g.unwrap());
                                let process_result = f32toi32(*g)
                                    .into_process_result(node_name);
                                __private_h_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = g {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                __private_h_tx.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "i";
                        s.spawn(|| {
                            let h = __private_h_rx.wait();
                            if h.is_ok() {
                                let (h) = (h.unwrap());
                                let process_result = sum(3 + 14, *h)
                                    .into_process_result(node_name);
                                __private_i_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = h {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                __private_i_tx.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "g_str_1";
                        s.spawn(|| {
                            let g = __private_g_rx.wait();
                            if g.is_ok() {
                                let (g) = (g.unwrap());
                                let process_result = g
                                    .to_string()
                                    .into_process_result(node_name);
                                __private_g_str_1_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = g {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                __private_g_str_1_tx.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "g_str";
                        s.spawn(|| {
                            let g = __private_g_rx.wait();
                            if g.is_ok() {
                                let (g) = (g.unwrap());
                                let process_result = { g.to_string() }
                                    .into_process_result(node_name);
                                __private_g_str_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = g {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                __private_g_str_tx.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "g_str_array";
                        s.spawn(|| {
                            let g_str_1 = __private_g_str_1_rx.wait();
                            let cic = __private_cic_rx.wait();
                            if g_str_1.is_ok() && cic.is_ok() {
                                let (g_str_1, cic) = (g_str_1.unwrap(), cic.unwrap());
                                let process_result = [
                                    g_str_1.deref().clone(),
                                    "Hi!".to_string(),
                                    cic.to_string(),
                                ]
                                    .into_process_result(node_name);
                                __private_g_str_array_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = g_str_1 {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                if let Err(e) = cic {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                __private_g_str_array_tx.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "out";
                        s.spawn(|| {
                            let h = __private_h_rx.wait();
                            if h.is_ok() {
                                let (h) = (h.unwrap());
                                let process_result = double(*h)
                                    .into_process_result(node_name);
                                __private_out_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = h {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                __private_out_tx.set(Err(joined_err));
                            }
                        });
                    }
                    (
                        __private_out_rx.wait(),
                        __private_e_rx.wait(),
                        __private_d_rx.wait(),
                        __private_g_str_array_rx.wait(),
                    )
                })
            },
            "digraph {graph [rankdir=\"TB\"];\ncic [shape=box label=\"cic\"]\nd [shape=box label=\"d\"]\ne [shape=box label=\"e\"]\nf [shape=box label=\"f\"]\ng [shape=box label=\"g\"]\nh [shape=box label=\"h\"]\ng -> h\ni [shape=box label=\"i\"]\nh -> i\ng_str_1 [shape=box label=\"g_str_1\"]\ng -> g_str_1\ng_str [shape=box label=\"g_str\"]\ng -> g_str\ng_str_array [shape=box label=\"g_str_array\"]\ng_str_1 -> g_str_array\ncic -> g_str_array\nout [shape=box label=\"out\"]\nh -> out\n___process_output [label=Output]\nout -> ___process_output\ne -> ___process_output\nd -> ___process_output\ng_str_array -> ___process_output\n}\n",
        )
    };
    let (a, b, c, d) = operation.exe();
    let err_dag = operation
        .visualize_errors([
            &a.map(|_| ()),
            &b.map(|_| ()),
            &c.map(|_| ()),
            &d.map(|_| ()),
        ]);
    {
        ::std::io::_print(format_args!("{0}\n", err_dag));
    };
}
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
fn mult(a: i32, b: i32) -> i32 {
    a * b
}
fn div(a: i32, b: i32) -> Result<f32, &'static str> {
    if b == 0 { Err("Division by zero!") } else { Ok(a as f32 / b as f32) }
}
fn double(input: i32) -> i32 {
    input * 2
}
