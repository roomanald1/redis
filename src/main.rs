use redis::{Commands};

fn main() -> Result<(), String> {
    println!("Hello, world!");
    let connection_str = "redis://ronnie:Prapr7fU_@redis-13230.c338.eu-west-2-1.ec2.redns.redis-cloud.com:13230";
    let client = redis::Client::open(connection_str).map_err(|e| e.to_string())?;
    let mut connection = client.get_connection().map_err(|e| e.to_string())?;
    let _: () = connection.set("key1".to_string(), "my Test value".to_string()).map_err(|e| e.to_string())?;
    let result: String = connection.get("key1".to_string()).map_err(|e| e.to_string())?;
    println!("getvalue = {result}");
    Ok(())
}
