(window.webpackJsonp=window.webpackJsonp||[]).push([[2],{14:function(o,e,n){"use strict";n.r(e),n.d(e,"start_ipfs",(function(){return s})),n.d(e,"write_file",(function(){return c}));let l=window.Ipfs;async function s(){let o=await l.create();console.log("IPFS"),console.log(l),console.log("IPFS client"),console.log(o);const{id:e,agentVersion:n,protocolVersion:s}=await o.id();console.log("id"),console.log(e),console.log("agentVersion"),console.log(n),console.log("protocolVersion"),console.log(s),console.log("files");for await(const e of o.files.ls("/"))console.log(e.name)}async function c(o,e){const n=window.Ipfs,l=await n.create(),{id:s,agentVersion:c,protocolVersion:i}=await l.id();console.log("id"),console.log(s),await l.files.write("/hello-world",(new TextEncoder).encode("Hello, world!"),{create:!0})}}}]);