use std::collections::HashMap;
use crate::redis;

fn parseReplica(replicaof: String, mut config: HashMap<String, String>) -> HashMap<String, String>{
    let mut parsedReplicaId = replicaof.split(" ");
    if parsedReplicaId.clone().count() == 2 {
        let ip = parsedReplicaId.next().unwrap();
        let port = parsedReplicaId.next().unwrap().parse::<u32>().unwrap();


        config.insert("role".to_string(), "slave".to_string());
        config.insert("master_host".to_string(), ip.to_string());
        config.insert("master_port".to_string(), port.to_string());
        config
    }
    else {
        config
    }


}
pub fn parseArgs(args: Vec<String>) -> HashMap<String, String>{
    let mut config = redis::getDefaultConfig();
    for mut i in 1..args.len(){
        if args[i-1] == "--port"{

            config.insert("port".to_string(), args[i].clone());

            i += 1;
        }
        if args[i-1] == "--replicaof"{

            config = parseReplica(args[i].clone(), config);
            i+=1;
        }

    }

    return config;
}