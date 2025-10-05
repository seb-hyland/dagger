use dagger_lib::prelude::*;

fn main() {
    fn failing_function() -> NodeResult<String> {
        Err(NodeError::msg("Function failed!"))
    }

    let result = dagger! {
        a :: Ok(5);
        b :: Ok(a + 3);
        c :: Ok(a - 3);
        d :: Ok(b + c);
        return d;
    }
    .exe();
    println!("{result:#?}");
}
