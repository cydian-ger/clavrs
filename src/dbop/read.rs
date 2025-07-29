use evmap::ReadHandle;
use super::op_type::*;

pub fn execute_read(
    read_handle: &ReadHandle<String, String>,
    operation: ReadOp,
) -> Result<String, String> {

    let ret: String;
    match operation {
        ReadOp::Get(keys) => {
            let mut get: Vec<Vec<String>> = Vec::new();
            for key in keys {
                if let Some(results) = read_handle.get(&key[..]) {
                    get.push((&*results).iter().map(|x| x.clone()).collect());
                } else {
                    get.push(Vec::new());
                }
            }
            ret = format!("{:?}", get);
        }

        // Exists []
        ReadOp::Exists(keys) => {
            let exists: Vec<bool> = keys
                .into_iter()
                .map(|x| read_handle.contains_key(&x))
                .collect();
            ret = format!("{:?}", exists);
        }

        // Has
        ReadOp::Has(keys, value) => {
            let has: Vec<bool> = keys
                .into_iter()
                .map(|key| read_handle.contains_value(&key, &value))
                .collect();
            ret = format!("{:?}", has);
        }
    };

    return Ok(ret);
}