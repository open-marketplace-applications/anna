(window.webpackJsonp=window.webpackJsonp||[]).push([[2],{8:function(o,n,l){"use strict";l.r(n),l.d(n,"start_ipfs",(function(){return s}));let e=window.Ipfs;async function s(){let o=await e.create();window.ipfs_client=o,console.log("IPFS"),console.log(e),console.log("IPFS client"),console.log(o);const{id:n,agentVersion:l,protocolVersion:s}=await o.id();console.log("id"),console.log(n),console.log("agentVersion"),console.log(l),console.log("protocolVersion"),console.log(s),console.log("files");for await(const n of o.files.ls("/"))console.log(n.name)}}}]);