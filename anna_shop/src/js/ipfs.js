export async function write_file(path, content) {

    const client = window.ipfs_client;
    console.log("client", client);

    if (client) {
        const { id, agentVersion, protocolVersion } = await client.id()

        console.log("id");
        console.log(id);
        await client.files.write('/hello-world', new TextEncoder().encode('Hello, world!'), { create: true })

    }


}
export async function get_published_products() {

    const client = window.ipfs_client;
    console.log("client", client);

    if (client) {
        let products = []

        for await (const file of client.files.ls('/')) {
            console.log(file.name)
            products.push(file.name)
        }
        console.log("resolve products", products);
        return products;
    } else {
        console.log("error loading files");
        return [];
    }
}

export function test() {

    return "hello world!"
}  