let IPFS = window.Ipfs;

export async function start_ipfs () {
    let client = await IPFS.create()
    window.ipfs_client = client;
    console.log("IPFS");
    console.log(IPFS);
    console.log("IPFS client");
    console.log(client);
    
    const { id, agentVersion, protocolVersion } = await client.id()
    console.log("id");
    console.log(id);
    console.log("agentVersion");
    console.log(agentVersion);
    console.log("protocolVersion");
    console.log(protocolVersion);
  
    console.log("files");
    for await (const file of client.files.ls('/')) {
      console.log(file.name)
    }
  
    // console.log("files");
    // for await (const file of client.files.ls('/')) {
    //   console.log(file.name)
    // }
  }
