(()=>{"use strict";var e,t,r,n,o,a,i={557:(e,t,r)=>{r.a(e,(async(n,o)=>{try{r.d(t,{Ih:()=>j,Or:()=>T,gV:()=>S,gk:()=>O,ug:()=>I,yq:()=>k});var a=r(367);e=r.hmd(e);var i=n([a]);a=(i.then?(await i)():i)[0];const c=new Array(32).fill(void 0);function s(e){return c[e]}c.push(void 0,null,!0,!1);let l=c.length;function u(e){e<36||(c[e]=l,l=e)}function d(e){const t=s(e);return u(e),t}let f=new("undefined"==typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});f.decode();let h=null;function _(){return null!==h&&h.buffer===a.memory.buffer||(h=new Uint8Array(a.memory.buffer)),h}function p(e,t){return f.decode(_().subarray(e,e+t))}let g=0;function b(e,t){const r=t(1*e.length);return _().set(e,r/1),g=e.length,r}let w=null;function y(){return null!==w&&w.buffer===a.memory.buffer||(w=new Int32Array(a.memory.buffer)),w}let m=null;function v(e){l===c.length&&c.push(c.length+1);const t=l;return l=c[t],c[t]=e,t}let x=new("undefined"==typeof TextEncoder?(0,e.require)("util").TextEncoder:TextEncoder)("utf-8");const E="function"==typeof x.encodeInto?function(e,t){return x.encodeInto(e,t)}:function(e,t){const r=x.encode(e);return t.set(r),{read:e.length,written:r.length}};function A(e,t,r){if(void 0===r){const r=x.encode(e),n=t(r.length);return _().subarray(n,n+r.length).set(r),g=r.length,n}let n=e.length,o=t(n);const a=_();let i=0;for(;i<n;i++){const t=e.charCodeAt(i);if(t>127)break;a[o+i]=t}if(i!==n){0!==i&&(e=e.slice(i)),o=r(o,n,n=i+3*e.length);const t=_().subarray(o+i,o+n);i+=E(e,t).written}return g=i,o}class S{static __wrap(e){const t=Object.create(S.prototype);return t.ptr=e,t}__destroy_into_raw(){const e=this.ptr;return this.ptr=0,e}free(){const e=this.__destroy_into_raw();a.__wbg_app_free(e)}static new(e,t){const r=a.app_new(e,t);return S.__wrap(r)}detect(e){try{const c=a.__wbindgen_add_to_stack_pointer(-16),s=b(e,a.__wbindgen_malloc),l=g;a.app_detect(c,this.ptr,s,l);var t=y()[c/4+0],r=y()[c/4+1],n=(o=t,i=r,(null!==m&&m.buffer===a.memory.buffer||(m=new Uint32Array(a.memory.buffer)),m).subarray(o/4,o/4+i)).slice();return a.__wbindgen_free(t,4*r),n}finally{a.__wbindgen_add_to_stack_pointer(16)}var o,i}}function j(){return v(new Error)}function k(e,t){const r=A(s(t).stack,a.__wbindgen_malloc,a.__wbindgen_realloc),n=g;y()[e/4+1]=n,y()[e/4+0]=r}function O(e,t){try{console.error(p(e,t))}finally{a.__wbindgen_free(e,t)}}function I(e){d(e)}function T(e,t){throw new Error(p(e,t))}o()}catch(P){o(P)}}))},341:(e,t,r)=>{r.a(e,(async(e,t)=>{try{var n=r(557),o=e([n]);n=(o.then?(await o)():o)[0];const a=128,i=96,c=8;let s=n.gV.new(a,i);console.log("wasabi.App.new");let l=document.getElementById("video");navigator.mediaDevices.getUserMedia({audio:!1,video:{width:{ideal:a*c},height:{ideal:i*c}}}).then((function(e){l.srcObject=e}));let u=document.getElementById("hidden-canvas");u.width=a,u.height=i;let d=u.getContext("2d"),f=document.getElementById("display-canvas");f.width=a*c,f.height=i*c;let h=f.getContext("2d");function _(){d.drawImage(l,0,0,u.width,u.height),h.drawImage(l,0,0,f.width,f.height);let e=d.getImageData(0,0,a,i).data,t=s.detect(Uint8Array.from(e));t.length>0&&(h.beginPath(),h.rect(t[0]*c,t[1]*c,(t[2]-t[0])*c,(t[3]-t[1])*c),h.fillStyle="#00FF00AA",h.fill(),h.closePath()),requestAnimationFrame(_)}_(),t()}catch(p){t(p)}}))},367:(e,t,r)=>{r.a(e,(async(n,o)=>{try{var a,i=n([a=r(557)]),[a]=i.then?(await i)():i;await r.v(t,e.id,"1062296af1003126ef48",{"./index_bg.js":{__wbg_new_693216e109162396:a.Ih,__wbg_stack_0ddaca5d1abfb52f:a.yq,__wbg_error_09919627ac0992f5:a.gk,__wbindgen_object_drop_ref:a.ug,__wbindgen_throw:a.Or}}),o()}catch(e){o(e)}}),1)}},c={};function s(e){var t=c[e];if(void 0!==t)return t.exports;var r=c[e]={id:e,loaded:!1,exports:{}};return i[e](r,r.exports,s),r.loaded=!0,r.exports}e="function"==typeof Symbol?Symbol("webpack then"):"__webpack_then__",t="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",r="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",n=e=>{e&&(e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},o=e=>!--e.r&&e(),a=(e,t)=>e?e.push(t):o(t),s.a=(i,c,s)=>{var l,u,d,f=s&&[],h=i.exports,_=!0,p=!1,g=(t,r,n)=>{p||(p=!0,r.r+=t.length,t.map(((t,o)=>t[e](r,n))),p=!1)},b=new Promise(((e,t)=>{d=t,u=()=>(e(h),n(f),f=0)}));b[t]=h,b[e]=(e,t)=>{if(_)return o(e);l&&g(l,e,t),a(f,e),b.catch(t)},i.exports=b,c((i=>{var c;l=(i=>i.map((i=>{if(null!==i&&"object"==typeof i){if(i[e])return i;if(i.then){var c=[];i.then((e=>{s[t]=e,n(c),c=0}),(e=>{s[r]=e,n(c),c=0}));var s={};return s[e]=(e,t)=>(a(c,e),i.catch(t)),s}}var l={};return l[e]=e=>o(e),l[t]=i,l})))(i);var s=()=>l.map((e=>{if(e[r])throw e[r];return e[t]})),u=new Promise(((e,t)=>{(c=()=>e(s)).r=0,g(l,c,t)}));return c.r?u:s()}),(e=>(e&&d(b[r]=e),u()))),_=!1},s.d=(e,t)=>{for(var r in t)s.o(t,r)&&!s.o(e,r)&&Object.defineProperty(e,r,{enumerable:!0,get:t[r]})},s.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),s.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),s.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),s.v=(e,t,r,n)=>{var o=fetch(s.p+""+r+".module.wasm");return"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(o,n).then((t=>Object.assign(e,t.instance.exports))):o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,n))).then((t=>Object.assign(e,t.instance.exports)))},(()=>{var e;s.g.importScripts&&(e=s.g.location+"");var t=s.g.document;if(!e&&t&&(t.currentScript&&(e=t.currentScript.src),!e)){var r=t.getElementsByTagName("script");r.length&&(e=r[r.length-1].src)}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),s.p=e})(),s(341)})();