#![feature(prelude_import)]
#![feature(custom_inner_attributes)]
#![rustfmt::skip]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use dagger::dagger;
fn main() {
    let input1 = || 3;
    let input2 = || 5;
    let f32toi32 = |v: f32| v.floor() as i32;
    let operation = {
        use dagger::prelude::*;
        Graph::new(
            || {
                let input1_data = ProcessData::new();
                let input2_data = ProcessData::new();
                let c_data = ProcessData::new();
                let d_data = ProcessData::new();
                let e_data = ProcessData::new();
                let f_data = ProcessData::new();
                let g_data = ProcessData::new();
                let h_data = ProcessData::new();
                let i_data = ProcessData::new();
                let g_str_data = ProcessData::new();
                let print_g_data = ProcessData::new();
                let out_data = ProcessData::new();
                ::std::thread::scope(|s| {
                    {
                        let node_name = "input1";
                        let input1_data_c = &input1_data;
                        s.spawn(|| {
                            if true {
                                let () = ();
                                let process_result = input1()
                                    .into_process_result(node_name);
                                input1_data_c.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                input1_data_c.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "input2";
                        let input2_data_c = &input2_data;
                        s.spawn(|| {
                            if true {
                                let () = ();
                                let process_result = input2()
                                    .into_process_result(node_name);
                                input2_data_c.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                input2_data_c.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "c";
                        let input1_data_c = &input1_data;
                        let input2_data_c = &input2_data;
                        let c_data_c = &c_data;
                        s.spawn(|| {
                            let input1 = input1_data_c.wait();
                            let input2 = input2_data_c.wait();
                            if input1.is_ok() && input2.is_ok() {
                                let (input1, input2) = (input1.unwrap(), input2.unwrap());
                                let process_result = sum(input1, input2)
                                    .into_process_result(node_name);
                                c_data_c.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = input1 {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                if let Err(e) = input2 {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                c_data_c.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "d";
                        let input1_data_c = &input1_data;
                        let input2_data_c = &input2_data;
                        let d_data_c = &d_data;
                        s.spawn(|| {
                            let input1 = input1_data_c.wait();
                            let input2 = input2_data_c.wait();
                            if input1.is_ok() && input2.is_ok() {
                                let (input1, input2) = (input1.unwrap(), input2.unwrap());
                                let process_result = mult(input1, input2)
                                    .into_process_result(node_name);
                                d_data_c.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = input1 {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                if let Err(e) = input2 {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                d_data_c.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "e";
                        let c_data_c = &c_data;
                        let d_data_c = &d_data;
                        let e_data_c = &e_data;
                        s.spawn(|| {
                            let c = c_data_c.wait();
                            let d = d_data_c.wait();
                            if c.is_ok() && d.is_ok() {
                                let (c, d) = (c.unwrap(), d.unwrap());
                                let process_result = sum(c, d)
                                    .into_process_result(node_name);
                                e_data_c.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = c {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                if let Err(e) = d {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                e_data_c.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "f";
                        let d_data_c = &d_data;
                        let f_data_c = &f_data;
                        s.spawn(|| {
                            let d = d_data_c.wait();
                            if d.is_ok() {
                                let (d) = (d.unwrap());
                                let process_result = double(d)
                                    .into_process_result(node_name);
                                f_data_c.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = d {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                f_data_c.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "g";
                        let e_data_c = &e_data;
                        let f_data_c = &f_data;
                        let g_data_c = &g_data;
                        s.spawn(|| {
                            let e = e_data_c.wait();
                            let f = f_data_c.wait();
                            if e.is_ok() && f.is_ok() {
                                let (e, f) = (e.unwrap(), f.unwrap());
                                let process_result = div(e, f)
                                    .into_process_result(node_name);
                                g_data_c.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = e {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                if let Err(e) = f {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                g_data_c.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "h";
                        let g_data_c = &g_data;
                        let h_data_c = &h_data;
                        s.spawn(|| {
                            let g = g_data_c.wait();
                            if g.is_ok() {
                                let (g) = (g.unwrap());
                                let process_result = f32toi32(g)
                                    .into_process_result(node_name);
                                h_data_c.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = g {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                h_data_c.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "i";
                        let input1_data_c = &input1_data;
                        let input2_data_c = &input2_data;
                        let i_data_c = &i_data;
                        s.spawn(|| {
                            let input1 = input1_data_c.wait();
                            let input2 = input2_data_c.wait();
                            if input1.is_ok() && input2.is_ok() {
                                let (input1, input2) = (input1.unwrap(), input2.unwrap());
                                let process_result = sum(input1, input2)
                                    .into_process_result(node_name);
                                i_data_c.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = input1 {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                if let Err(e) = input2 {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                i_data_c.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "g_str";
                        let g_data_c = &g_data;
                        let g_str_data_c = &g_str_data;
                        s.spawn(|| {
                            let g = g_data_c.wait();
                            if g.is_ok() {
                                let (g) = (g.unwrap());
                                let process_result = as_string(g)
                                    .into_process_result(node_name);
                                g_str_data_c.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = g {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                g_str_data_c.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "print_g";
                        let g_str_data_c = &g_str_data;
                        let print_g_data_c = &print_g_data;
                        s.spawn(|| {
                            let g_str = g_str_data_c.wait();
                            if g_str.is_ok() {
                                let (g_str) = (g_str.unwrap());
                                let process_result = print_string(g_str)
                                    .into_process_result(node_name);
                                print_g_data_c.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = g_str {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                print_g_data_c.set(Err(joined_err));
                            }
                        });
                    }
                    {
                        let node_name = "out";
                        let h_data_c = &h_data;
                        let out_data_c = &out_data;
                        s.spawn(|| {
                            let h = h_data_c.wait();
                            if h.is_ok() {
                                let (h) = (h.unwrap());
                                let process_result = double(h)
                                    .into_process_result(node_name);
                                out_data_c.set(process_result);
                            } else {
                                let mut joined_err = ProcessError::default();
                                if let Err(e) = h {
                                    e.push_error(node_name, &mut joined_err);
                                }
                                out_data_c.set(Err(joined_err));
                            }
                        });
                    }
                    let out_data_c = &out_data;
                    let e_data_c = &e_data;
                    let c_data_c = &c_data;
                    let g_str_data_c = &g_str_data;
                    (
                        out_data_c.wait(),
                        e_data_c.wait(),
                        c_data_c.wait(),
                        g_str_data_c.wait(),
                    )
                })
            },
            "digraph {graph [rankdir=\"TB\"];\ninput1 [shape=box label=\"input1: input1\"]\ninput2 [shape=box label=\"input2: input2\"]\nc [shape=box label=\"c: sum\"]\ninput1 -> c\ninput2 -> c\nd [shape=box label=\"d: mult\"]\ninput1 -> d\ninput2 -> d\ne [shape=box label=\"e: sum\"]\nc -> e\nd -> e\nf [shape=box label=\"f: double\"]\nd -> f\ng [shape=box label=\"g: div\"]\ne -> g\nf -> g\nh [shape=box label=\"h: f32toi32\"]\ng -> h\ni [shape=box label=\"i: sum\"]\ninput1 -> i\ninput2 -> i\ng_str [shape=box label=\"g_str: as_string\"]\ng -> g_str\nprint_g [shape=box label=\"print_g: print_string\"]\ng_str -> print_g\nout [shape=box label=\"out: double\"]\nh -> out\ncrapper___[label=Output]\nout -> crapper___\ne -> crapper___\nc -> crapper___\ng_str -> crapper___\n}\n",
        )
    };
    let result = operation.exe();
    match result {
        tmp => {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3} = {4:#?}\n",
                        "dagger-test/src/main.rs",
                        26u32,
                        5u32,
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
fn div(a: i32, b: i32) -> f32 {
    a as f32 / b as f32
}
fn double(input: i32) -> i32 {
    input * 2
}
fn as_string(input: f32) -> String {
    input.to_string()
}
fn print_string(input: String) {
    {
        ::std::io::_print(format_args!("{0}\n", input));
    }
}
