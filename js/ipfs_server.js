export async function start_ipfs () {
    const IPFS = window.Ipfs;
    const client = await IPFS.create()
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
  
    console.log("write");
    await client.files.write('/hello-world', new TextEncoder().encode('Hello, world!'), { create: true })
    
    console.log("files");
    for await (const file of client.files.ls('/')) {
      console.log(file.name)
    }
  }
  