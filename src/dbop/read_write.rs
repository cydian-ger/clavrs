use super::op_type::*;
use evmap::WriteHandle;

pub fn execute_read_write(
    write_handle: &mut WriteHandle<String, String>,
    operation: ReadWriteOp,
) -> Result<String, String> {
    let res: String;
    match operation {
        ReadWriteOp::Pop(key) => {
            let _pop: Option<Vec<String>>;
            match write_handle.get(&key) {
                Some(values) => {
                    let popped: Vec<String> = values.iter().map(|x| x.clone()).collect();
                    res = format!("{:?}", popped);
                }
                None => {
                    let empty: Vec<String> = vec![];
                    res = format!("{:?}", empty);
                }
            }
            write_handle.empty(key);

        }
    }

    return Ok(res);
}
