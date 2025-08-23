#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use dagger::dagger;
fn main() {
    let input1 = || 3;
    let input2 = || 0;
    let f32toi32 = |v: f32| -> Result<i32, String> { Ok(v.floor() as i32) };
    let operation = {
        use dagger::prelude::*;
        Graph::new(
            || {
                let _input1_data = ProcessData::default();
                let _input2_data = ProcessData::default();
                let _c_data = ProcessData::default();
                let _d_data = ProcessData::default();
                let _e_data = ProcessData::default();
                let _f_data = ProcessData::default();
                let _g_data = ProcessData::default();
                let _h_data = ProcessData::default();
                let _i_data = ProcessData::default();
                let _g_str_1_data = ProcessData::default();
                let _g_str_data = ProcessData::default();
                let _out_data = ProcessData::default();
                let (_input1_data_tx, _input1_data_rx) = _input1_data.channel();
                let (_input2_data_tx, _input2_data_rx) = _input2_data.channel();
                let (_c_data_tx, _c_data_rx) = _c_data.channel();
                let (_d_data_tx, _d_data_rx) = _d_data.channel();
                let (_e_data_tx, _e_data_rx) = _e_data.channel();
                let (_f_data_tx, _f_data_rx) = _f_data.channel();
                let (_g_data_tx, _g_data_rx) = _g_data.channel();
                let (_h_data_tx, _h_data_rx) = _h_data.channel();
                let (_i_data_tx, _i_data_rx) = _i_data.channel();
                let (_g_str_1_data_tx, _g_str_1_data_rx) = _g_str_1_data.channel();
                let (_g_str_data_tx, _g_str_data_rx) = _g_str_data.channel();
                let (_out_data_tx, _out_data_rx) = _out_data.channel();
                ::std::thread::scope(|s| {
                    {
                        let node_name = "input1";
                        s.spawn(|| {
                            if true {
                                let () = ();
                                let process_result = input1()
                                    .into_process_result(node_name);
                                _input1_data_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                _input1_data_tx.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "input2";
                        s.spawn(|| {
                            if true {
                                let () = ();
                                let process_result = input2()
                                    .into_process_result(node_name);
                                _input2_data_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                _input2_data_tx.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "c";
                        s.spawn(|| {
                            let input1 = _input1_data_rx.wait();
                            let input2 = _input2_data_rx.wait();
                            if input1.is_ok() && input2.is_ok() {
                                let (input1, input2) = (input1.unwrap(), input2.unwrap());
                                let process_result = sum(*input1, *input2)
                                    .into_process_result(node_name);
                                _c_data_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = input1 {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                if let Err(e) = input2 {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                _c_data_tx.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "d";
                        s.spawn(|| {
                            let input1 = _input1_data_rx.wait();
                            let input2 = _input2_data_rx.wait();
                            if input1.is_ok() && input2.is_ok() {
                                let (input1, input2) = (input1.unwrap(), input2.unwrap());
                                let process_result = mult(*input1, *input2)
                                    .into_process_result(node_name);
                                _d_data_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = input1 {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                if let Err(e) = input2 {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                _d_data_tx.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "e";
                        s.spawn(|| {
                            let c = _c_data_rx.wait();
                            let d = _d_data_rx.wait();
                            if c.is_ok() && d.is_ok() {
                                let (c, d) = (c.unwrap(), d.unwrap());
                                let process_result = sum(*c + 2, *d)
                                    .into_process_result(node_name);
                                _e_data_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = c {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                if let Err(e) = d {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                _e_data_tx.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "f";
                        s.spawn(|| {
                            let d = _d_data_rx.wait();
                            if d.is_ok() {
                                let (d) = (d.unwrap());
                                let process_result = double(*d)
                                    .into_process_result(node_name);
                                _f_data_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = d {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                _f_data_tx.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "g";
                        s.spawn(|| {
                            let e = _e_data_rx.wait();
                            let f = _f_data_rx.wait();
                            if e.is_ok() && f.is_ok() {
                                let (e, f) = (e.unwrap(), f.unwrap());
                                let process_result = div(*e, *f)
                                    .into_process_result(node_name);
                                _g_data_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = e {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                if let Err(e) = f {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                _g_data_tx.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "h";
                        s.spawn(|| {
                            let g = _g_data_rx.wait();
                            if g.is_ok() {
                                let (g) = (g.unwrap());
                                let process_result = f32toi32(*g)
                                    .into_process_result(node_name);
                                _h_data_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = g {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                _h_data_tx.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "i";
                        s.spawn(|| {
                            let input1 = _input1_data_rx.wait();
                            let input2 = _input2_data_rx.wait();
                            if input1.is_ok() && input2.is_ok() {
                                let (input1, input2) = (input1.unwrap(), input2.unwrap());
                                let process_result = sum(*input1 + 14, *input2)
                                    .into_process_result(node_name);
                                _i_data_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = input1 {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                if let Err(e) = input2 {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                _i_data_tx.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "g_str_1";
                        s.spawn(|| {
                            let g = _g_data_rx.wait();
                            if g.is_ok() {
                                let (g) = (g.unwrap());
                                let process_result = noop(g.to_string())
                                    .into_process_result(node_name);
                                _g_str_1_data_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = g {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                _g_str_1_data_tx.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "g_str";
                        s.spawn(|| {
                            let g = _g_data_rx.wait();
                            let g_str_1 = _g_str_1_data_rx.wait();
                            if g.is_ok() && g_str_1.is_ok() {
                                let (g, g_str_1) = (g.unwrap(), g_str_1.unwrap());
                                let process_result = noop(g.to_string().find(&*g_str_1))
                                    .into_process_result(node_name);
                                _g_str_data_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = g {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                if let Err(e) = g_str_1 {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                _g_str_data_tx.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "out";
                        s.spawn(|| {
                            let h = _h_data_rx.wait();
                            if h.is_ok() {
                                let (h) = (h.unwrap());
                                let process_result = double(*h)
                                    .into_process_result(node_name);
                                _out_data_tx.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = h {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                _out_data_tx.set(Err(joined_err));
                            }
                        });
                    }
                    (
                        _out_data_rx.wait(),
                        _e_data_rx.wait(),
                        _d_data_rx.wait(),
                        _g_str_data_rx.wait(),
                    )
                })
            },
            "digraph {graph [rankdir=\"TB\"];\ninput1 [shape=box label=\"input1: input1\"]\ninput2 [shape=box label=\"input2: input2\"]\nc [shape=box label=\"c: sum\"]\ninput1 -> c\ninput2 -> c\nd [shape=box label=\"d: mult\"]\ninput1 -> d\ninput2 -> d\ne [shape=box label=\"e: sum\"]\nc -> e\nd -> e\nf [shape=box label=\"f: double\"]\nd -> f\ng [shape=box label=\"g: div\"]\ne -> g\nf -> g\nh [shape=box label=\"h: f32toi32\"]\ng -> h\ni [shape=box label=\"i: sum\"]\ninput1 -> i\ninput2 -> i\ng_str_1 [shape=box label=\"g_str_1: noop\"]\ng -> g_str_1\ng_str [shape=box label=\"g_str: noop\"]\ng -> g_str\ng_str_1 -> g_str\nout [shape=box label=\"out: double\"]\nh -> out\ncrapper___[label=Output]\nout -> crapper___\ne -> crapper___\nd -> crapper___\ng_str -> crapper___\n}\n",
        )
    };
    let result = operation.exe();
    {
        ::std::io::_print(format_args!("{0}\n", operation.dot()));
    };
    let _ = match result {
        tmp => {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3} = {4:#?}\n",
                        "dagger-test/src/main.rs",
                        24u32,
                        13u32,
                        "result",
                        &&tmp as &dyn ::std::fmt::Debug,
                    ),
                );
            };
            tmp
        }
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
fn noop<T>(input: T) -> T {
    input
}
