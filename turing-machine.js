let L=0,U=`string`,M=1,V=`Object`,O=`utf-8`,K=null,S=`number`,N=`undefined`,T=`boolean`,X=4,Z=25,R=`function`,W=16,I=Array,P=Error,Y=Promise,Q=Uint8Array,J=undefined;var C=(async(a,b)=>{if(typeof Response===R&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===R){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var v=(a=>{const b=typeof a;if(b==S||b==T||a==K){return `${a}`};if(b==U){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==K){return `Symbol`}else{return `Symbol(${b})`}};if(b==R){const b=a.name;if(typeof b==U&&b.length>L){return `Function(${b})`}else{return `Function`}};if(I.isArray(a)){const b=a.length;let c=`[`;if(b>L){c+=v(a[L])};for(let d=M;d<b;d++){c+=`, `+ v(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>M){d=c[M]}else{return toString.call(a)};if(d==V){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return V}};if(a instanceof P){return `${a.name}: ${a.message}\n${a.stack}`};return d});var x=((a,c,d)=>{b.wasm_bindgen__convert__closures__invoke1_mut__h474cdd8586d31e29(a,c,k(d))});var E=((a,b)=>{});var z=((a,c)=>{b.wasm_bindgen__convert__closures__invoke0_mut__h09b8aeac5250f6e3(a,c)});var B=((a,c,d,e)=>{b.wasm_bindgen__convert__closures__invoke2_mut__h580bb2f6e961bc2c(a,c,k(d),k(e))});var q=(a=>{const b=d(a);p(a);return b});var k=(a=>{if(j===c.length)c.push(c.length+ M);const b=j;j=c[b];c[b]=a;return b});function A(a,c){try{return a.apply(this,c)}catch(a){b.__wbindgen_exn_store(k(a))}}var i=(()=>{if(h===K||h.byteLength===L){h=new Int32Array(b.memory.buffer)};return h});var e=(a=>a===J||a===K);var d=(a=>c[a]);var H=(async(a)=>{if(b!==J)return b;if(typeof a===N){a=new URL(`turing-machine_bg.wasm`,import.meta.url)};const c=D();if(typeof a===U||typeof Request===R&&a instanceof Request||typeof URL===R&&a instanceof URL){a=fetch(a)};E(c);const {instance:d,module:e}=await C(await a,c);return F(d,e)});var G=(a=>{if(b!==J)return b;const c=D();E(c);if(!(a instanceof WebAssembly.Module)){a=new WebAssembly.Module(a)};const d=new WebAssembly.Instance(a,c);return F(d,a)});var D=(()=>{const c={};c.wbg={};c.wbg.__wbg_log_8abd9363c81b1b3a=((a,b)=>{console.log(o(a,b))});c.wbg.__wbg_warn_04faf274d400addd=((a,b)=>{console.warn(o(a,b))});c.wbg.__wbg_deleteProgram_53a32852f245b839=((a,b)=>{d(a).deleteProgram(d(b))});c.wbg.__wbg_deleteProgram_d8d7fc79ba83b256=((a,b)=>{d(a).deleteProgram(d(b))});c.wbg.__wbindgen_number_get=((a,b)=>{const c=d(b);const f=typeof c===S?c:J;g()[a/8+ M]=e(f)?L:f;i()[a/X+ L]=!e(f)});c.wbg.__wbg_createProgram_88dbe21c0b682e1a=(a=>{const b=d(a).createProgram();return e(b)?L:k(b)});c.wbg.__wbg_createProgram_4eaf3b97b5747a62=(a=>{const b=d(a).createProgram();return e(b)?L:k(b)});c.wbg.__wbg_linkProgram_9a2d12d120d99917=((a,b)=>{d(a).linkProgram(d(b))});c.wbg.__wbg_linkProgram_33998194075d71fb=((a,b)=>{d(a).linkProgram(d(b))});c.wbg.__wbg_getProgramParameter_2a3735278367f8bc=((a,b,c)=>{const e=d(a).getProgramParameter(d(b),c>>>L);return k(e)});c.wbg.__wbg_getProgramParameter_35522a0bfdfaad27=((a,b,c)=>{const e=d(a).getProgramParameter(d(b),c>>>L);return k(e)});c.wbg.__wbg_getProgramInfoLog_0b7af4ad85fa52a4=((a,c,f)=>{const g=d(c).getProgramInfoLog(d(f));var h=e(g)?L:u(g,b.__wbindgen_malloc,b.__wbindgen_realloc);var j=r;i()[a/X+ M]=j;i()[a/X+ L]=h});c.wbg.__wbg_attachShader_b65b695055670cb5=((a,b,c)=>{d(a).attachShader(d(b),d(c))});c.wbg.__wbg_attachShader_47256b6b3d42a22e=((a,b,c)=>{d(a).attachShader(d(b),d(c))});c.wbg.__wbg_getProgramInfoLog_b81bc53188e286fa=((a,c,f)=>{const g=d(c).getProgramInfoLog(d(f));var h=e(g)?L:u(g,b.__wbindgen_malloc,b.__wbindgen_realloc);var j=r;i()[a/X+ M]=j;i()[a/X+ L]=h});c.wbg.__wbg_createVertexArrayOES_6a3c3a5a68201f8f=(a=>{const b=d(a).createVertexArrayOES();return e(b)?L:k(b)});c.wbg.__wbg_createVertexArray_51d51e1e1e13e9f6=(a=>{const b=d(a).createVertexArray();return e(b)?L:k(b)});c.wbg.__wbg_matches_07c564b5b4101cf2=(a=>{const b=d(a).matches;return b});c.wbg.__wbg_location_56243dba507f472d=(a=>{const b=d(a).location;return k(b)});c.wbg.__wbg_search_6c3c472e076ee010=function(){return A(((a,c)=>{const e=d(c).search;const f=u(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=r;i()[a/X+ M]=g;i()[a/X+ L]=f}),arguments)};c.wbg.__wbg_href_d62a28e4fc1ab948=function(){return A(((a,c)=>{const e=d(c).href;const f=u(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=r;i()[a/X+ M]=g;i()[a/X+ L]=f}),arguments)};c.wbg.__wbg_protocol_91948f5885595359=function(){return A(((a,c)=>{const e=d(c).protocol;const f=u(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=r;i()[a/X+ M]=g;i()[a/X+ L]=f}),arguments)};c.wbg.__wbg_host_15090f3de0544fea=function(){return A(((a,c)=>{const e=d(c).host;const f=u(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=r;i()[a/X+ M]=g;i()[a/X+ L]=f}),arguments)};c.wbg.__wbg_hostname_b77e5e70d6ff6236=function(){return A(((a,c)=>{const e=d(c).hostname;const f=u(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=r;i()[a/X+ M]=g;i()[a/X+ L]=f}),arguments)};c.wbg.__wbg_port_1b2b1249cacfca76=function(){return A(((a,c)=>{const e=d(c).port;const f=u(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=r;i()[a/X+ M]=g;i()[a/X+ L]=f}),arguments)};c.wbg.__wbg_origin_50aa482fa6784a0a=function(){return A(((a,c)=>{const e=d(c).origin;const f=u(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=r;i()[a/X+ M]=g;i()[a/X+ L]=f}),arguments)};c.wbg.__wbindgen_object_clone_ref=(a=>{const b=d(a);return k(b)});c.wbg.__wbindgen_string_new=((a,b)=>{const c=o(a,b);return k(c)});c.wbg.__wbg_setid_1984ee27e5075311=((a,b,c)=>{d(a).id=o(b,c)});c.wbg.__wbg_style_3801009b2339aa94=(a=>{const b=d(a).style;return k(b)});c.wbg.__wbg_setsize_7532844e2c9f5e10=((a,b)=>{d(a).size=b>>>L});c.wbg.__wbg_setautofocus_61b6a31b4866ad1f=((a,b)=>{d(a).autofocus=b!==L});c.wbg.__wbg_sethidden_0cbfa2481b57c377=((a,b)=>{d(a).hidden=b!==L});c.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new P();return k(a)});c.wbg.__wbg_stack_658279fe44541cf6=((a,c)=>{const e=d(c).stack;const f=u(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=r;i()[a/X+ M]=g;i()[a/X+ L]=f});c.wbg.__wbg_error_f851667af71bcfc6=((a,c)=>{let d;let e;try{d=a;e=c;console.error(o(a,c))}finally{b.__wbindgen_free(d,e,M)}});c.wbg.__wbindgen_object_drop_ref=(a=>{q(a)});c.wbg.__wbg_width_2931aaedd21f1fff=(a=>{const b=d(a).width;return b});c.wbg.__wbg_height_0d36fbbeb60b0661=(a=>{const b=d(a).height;return b});c.wbg.__wbg_clearColor_de587608b28bc7ed=((a,b,c,e,f)=>{d(a).clearColor(b,c,e,f)});c.wbg.__wbg_clearColor_7a7d04702f7e38e5=((a,b,c,e,f)=>{d(a).clearColor(b,c,e,f)});c.wbg.__wbg_clear_2ccea1f65b510c97=((a,b)=>{d(a).clear(b>>>L)});c.wbg.__wbg_clear_2db2efe323bfdf68=((a,b)=>{d(a).clear(b>>>L)});c.wbg.__wbg_disableVertexAttribArray_8dacd44e21adcaa2=((a,b)=>{d(a).disableVertexAttribArray(b>>>L)});c.wbg.__wbg_disableVertexAttribArray_6d57776c8f642f44=((a,b)=>{d(a).disableVertexAttribArray(b>>>L)});c.wbg.__wbg_scissor_c8ec3b1e053f3756=((a,b,c,e,f)=>{d(a).scissor(b,c,e,f)});c.wbg.__wbg_scissor_e8e41e1c0a9817c8=((a,b,c,e,f)=>{d(a).scissor(b,c,e,f)});c.wbg.__wbg_createTexture_9d0bb4d741b8ad76=(a=>{const b=d(a).createTexture();return e(b)?L:k(b)});c.wbg.__wbg_createTexture_1bf4d6fec570124b=(a=>{const b=d(a).createTexture();return e(b)?L:k(b)});c.wbg.__wbg_performance_2c295061c8b01e0b=(a=>{const b=d(a).performance;return e(b)?L:k(b)});c.wbg.__wbg_now_0cfdc90c97d0c24b=(a=>{const b=d(a).now();return b});c.wbg.__wbg_parentElement_c75962bc9997ea5f=(a=>{const b=d(a).parentElement;return e(b)?L:k(b)});c.wbg.__wbg_clientWidth_51ec21e3189f5656=(a=>{const b=d(a).clientWidth;return b});c.wbg.__wbg_clientHeight_09ec0b524d59c367=(a=>{const b=d(a).clientHeight;return b});c.wbg.__wbg_setwidth_a667a942dba6656e=((a,b)=>{d(a).width=b>>>L});c.wbg.__wbg_setheight_a747d440760fe5aa=((a,b)=>{d(a).height=b>>>L});c.wbg.__wbg_navigator_7c9103698acde322=(a=>{const b=d(a).navigator;return k(b)});c.wbg.__wbg_getBoundingClientRect_ac9db8cf97ca8083=(a=>{const b=d(a).getBoundingClientRect();return k(b)});c.wbg.__wbg_scrollTop_9e5ce77431551404=(a=>{const b=d(a).scrollTop;return b});c.wbg.__wbg_offsetTop_815aa9ab53b3cf18=(a=>{const b=d(a).offsetTop;return b});c.wbg.__wbg_height_bed51746e072a118=(a=>{const b=d(a).height;return b});c.wbg.__wbg_scrollLeft_ea915614eac6bbeb=(a=>{const b=d(a).scrollLeft;return b});c.wbg.__wbg_offsetLeft_3b7ae7e9baa5358a=(a=>{const b=d(a).offsetLeft;return b});c.wbg.__wbg_offsetWidth_4e9930121c69297f=(a=>{const b=d(a).offsetWidth;return b});c.wbg.__wbg_width_e0c6b79d8cdd8897=(a=>{const b=d(a).width;return b});c.wbg.__wbg_requestAnimationFrame_d082200514b6674d=function(){return A(((a,b)=>{const c=d(a).requestAnimationFrame(d(b));return c}),arguments)};c.wbg.__wbindgen_cb_drop=(a=>{const b=q(a).original;if(b.cnt--==M){b.a=L;return !0};const c=!1;return c});c.wbg.__wbg_instanceof_HtmlCanvasElement_da5f9efa0688cf6d=(a=>{let b;try{b=d(a) instanceof HTMLCanvasElement}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_devicePixelRatio_f9de7bddca0eaf20=(a=>{const b=d(a).devicePixelRatio;return b});c.wbg.__wbg_removeEventListener_5de660c02ed784e4=function(){return A(((a,b,c,e)=>{d(a).removeEventListener(o(b,c),d(e))}),arguments)};c.wbg.__wbg_clearInterval_080a47b47538d08c=((a,b)=>{d(a).clearInterval(b)});c.wbg.__wbg_blur_53431c003c82bf53=function(){return A((a=>{d(a).blur()}),arguments)};c.wbg.__wbg_getItem_ed8e218e51f1efeb=function(){return A(((a,c,f,g)=>{const h=d(c).getItem(o(f,g));var j=e(h)?L:u(h,b.__wbindgen_malloc,b.__wbindgen_realloc);var k=r;i()[a/X+ M]=k;i()[a/X+ L]=j}),arguments)};c.wbg.__wbg_localStorage_dbac11bd189e9fa0=function(){return A((a=>{const b=d(a).localStorage;return e(b)?L:k(b)}),arguments)};c.wbg.__wbg_setItem_d002ee486462bfff=function(){return A(((a,b,c,e,f)=>{d(a).setItem(o(b,c),o(e,f))}),arguments)};c.wbg.__wbg_altKey_612289acf855835c=(a=>{const b=d(a).altKey;return b});c.wbg.__wbg_shiftKey_48e8701355d8e2d4=(a=>{const b=d(a).shiftKey;return b});c.wbg.__wbg_isComposing_f41d219def91d438=(a=>{const b=d(a).isComposing;return b});c.wbg.__wbg_keyCode_dfa86be31f5ef90c=(a=>{const b=d(a).keyCode;return b});c.wbg.__wbg_preventDefault_24104f3f0a54546a=(a=>{d(a).preventDefault()});c.wbg.__wbg_matchMedia_12ef69056e32d0b3=function(){return A(((a,b,c)=>{const f=d(a).matchMedia(o(b,c));return e(f)?L:k(f)}),arguments)};c.wbg.__wbg_matches_0f7e350783b542c2=(a=>{const b=d(a).matches;return b});c.wbg.__wbg_files_e5c28ff6ab126f7b=(a=>{const b=d(a).files;return e(b)?L:k(b)});c.wbg.__wbg_length_b941879633a63ad8=(a=>{const b=d(a).length;return b});c.wbg.__wbg_type_8b3fde044d705ef3=((a,c)=>{const e=d(c).type;const f=u(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=r;i()[a/X+ M]=g;i()[a/X+ L]=f});c.wbg.__wbg_lastModified_711034410dfc02ad=(a=>{const b=d(a).lastModified;return b});c.wbg.__wbg_arrayBuffer_27cefaea55cbf063=(a=>{const b=d(a).arrayBuffer();return k(b)});c.wbg.__wbg_stopPropagation_55539cfa2506c867=(a=>{d(a).stopPropagation()});c.wbg.__wbg_new_d8a000788389a31e=(a=>{const b=new Q(d(a));return k(b)});c.wbg.__wbg_length_a5587d6cd79ab197=(a=>{const b=d(a).length;return b});c.wbg.__wbg_items_0076326dc6f1b7eb=(a=>{const b=d(a).items;return k(b)});c.wbg.__wbg_length_dd2eb44022569c32=(a=>{const b=d(a).length;return b});c.wbg.__wbg_get_135f0a95f49ed3ff=((a,b)=>{const c=d(a)[b>>>L];return e(c)?L:k(c)});c.wbg.__wbg_type_9f716e985ca0633c=((a,c)=>{const e=d(c).type;const f=u(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=r;i()[a/X+ M]=g;i()[a/X+ L]=f});c.wbg.__wbg_deltaMode_1c680147cfdba8a5=(a=>{const b=d(a).deltaMode;return b});c.wbg.__wbg_deltaX_84508d00a1050e70=(a=>{const b=d(a).deltaX;return b});c.wbg.__wbg_deltaY_64823169afb0335d=(a=>{const b=d(a).deltaY;return b});c.wbg.__wbg_ctrlKey_0a805df688b5bf42=(a=>{const b=d(a).ctrlKey;return b});c.wbg.__wbg_metaKey_d89287be4389a3c1=(a=>{const b=d(a).metaKey;return b});c.wbg.__wbg_shiftKey_8a070ab6169b5fa4=(a=>{const b=d(a).shiftKey;return b});c.wbg.__wbg_length_25c4aaeba8cfcc81=(a=>{const b=d(a).length;return b});c.wbg.__wbg_item_59a092aa0f27eab6=((a,b)=>{const c=d(a).item(b>>>L);return e(c)?L:k(c)});c.wbg.__wbg_identifier_da93d3d09ccdc54c=(a=>{const b=d(a).identifier;return b});c.wbg.__wbg_force_4dd0ab6e9ef993ec=(a=>{const b=d(a).force;return b});c.wbg.__wbg_pageX_8e76f76ea9375a85=(a=>{const b=d(a).pageX;return b});c.wbg.__wbg_pageY_a5a407b52fe202e7=(a=>{const b=d(a).pageY;return b});c.wbg.__wbg_innerHeight_2dd06d8cf68f1d7d=function(){return A((a=>{const b=d(a).innerHeight;return k(b)}),arguments)};c.wbg.__wbg_button_7a095234b69de930=(a=>{const b=d(a).button;return b});c.wbg.__wbg_clientX_1a480606ab0cabaa=(a=>{const b=d(a).clientX;return b});c.wbg.__wbg_clientY_9c7878f7faf3900f=(a=>{const b=d(a).clientY;return b});c.wbg.__wbg_new_e7fbaa407e13d590=(()=>{const a=new P();return k(a)});c.wbg.__wbg_stack_21698d2a5852e13e=((a,c)=>{const e=d(c).stack;const f=u(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=r;i()[a/X+ M]=g;i()[a/X+ L]=f});c.wbg.__wbg_error_e38422e56bbd072c=((a,c)=>{let d;let e;try{d=a;e=c;console.error(o(a,c))}finally{b.__wbindgen_free(d,e,M)}});c.wbg.__wbg_value_9423da9d988ee8cf=((a,c)=>{const e=d(c).value;const f=u(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=r;i()[a/X+ M]=g;i()[a/X+ L]=f});c.wbg.__wbg_instanceof_WebGlRenderingContext_ea632546035eecb1=(a=>{let b;try{b=d(a) instanceof WebGLRenderingContext}catch(a){b=!1}const c=b;return c});c.wbg.__wbindgen_string_get=((a,c)=>{const f=d(c);const g=typeof f===U?f:J;var h=e(g)?L:u(g,b.__wbindgen_malloc,b.__wbindgen_realloc);var j=r;i()[a/X+ M]=j;i()[a/X+ L]=h});c.wbg.__wbg_getSupportedExtensions_4eb3a5f14f552ce5=(a=>{const b=d(a).getSupportedExtensions();return e(b)?L:k(b)});c.wbg.__wbg_length_cace2e0b3ddc0502=(a=>{const b=d(a).length;return b});c.wbg.__wbg_instanceof_WebGl2RenderingContext_f921526c513bf717=(a=>{let b;try{b=d(a) instanceof WebGL2RenderingContext}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_getSupportedExtensions_fafc31aab913037d=(a=>{const b=d(a).getSupportedExtensions();return e(b)?L:k(b)});c.wbg.__wbg_getError_1e5ec1ec9e58b323=(a=>{const b=d(a).getError();return b});c.wbg.__wbg_getError_7191ad6ea53607fe=(a=>{const b=d(a).getError();return b});c.wbg.__wbg_createShader_9d7d388633caad18=((a,b)=>{const c=d(a).createShader(b>>>L);return e(c)?L:k(c)});c.wbg.__wbg_createShader_429776c9dd6fb87b=((a,b)=>{const c=d(a).createShader(b>>>L);return e(c)?L:k(c)});c.wbg.__wbg_shaderSource_f435f9b74440bb54=((a,b,c,e)=>{d(a).shaderSource(d(b),o(c,e))});c.wbg.__wbg_shaderSource_1cb7c64dc7d1a500=((a,b,c,e)=>{d(a).shaderSource(d(b),o(c,e))});c.wbg.__wbg_compileShader_d88d0a8cd9b72b4d=((a,b)=>{d(a).compileShader(d(b))});c.wbg.__wbg_compileShader_6bf78b425d5c98e1=((a,b)=>{d(a).compileShader(d(b))});c.wbg.__wbg_getShaderParameter_e8054f1d9026fb70=((a,b,c)=>{const e=d(a).getShaderParameter(d(b),c>>>L);return k(e)});c.wbg.__wbg_getShaderParameter_ac2727ae4fe7648e=((a,b,c)=>{const e=d(a).getShaderParameter(d(b),c>>>L);return k(e)});c.wbg.__wbg_getShaderInfoLog_979aafa403ffb252=((a,c,f)=>{const g=d(c).getShaderInfoLog(d(f));var h=e(g)?L:u(g,b.__wbindgen_malloc,b.__wbindgen_realloc);var j=r;i()[a/X+ M]=j;i()[a/X+ L]=h});c.wbg.__wbg_getShaderInfoLog_968b93e75477d725=((a,c,f)=>{const g=d(c).getShaderInfoLog(d(f));var h=e(g)?L:u(g,b.__wbindgen_malloc,b.__wbindgen_realloc);var j=r;i()[a/X+ M]=j;i()[a/X+ L]=h});c.wbg.__wbg_pixelStorei_5ec932ebefd00149=((a,b,c)=>{d(a).pixelStorei(b>>>L,c)});c.wbg.__wbg_pixelStorei_f3a24990aa352fc7=((a,b,c)=>{d(a).pixelStorei(b>>>L,c)});c.wbg.__wbg_texSubImage2D_f1a31f8045b7f831=function(){return A(((a,b,c,e,f,g,h,i,j,k)=>{d(a).texSubImage2D(b>>>L,c,e,f,g,h,i>>>L,j>>>L,d(k))}),arguments)};c.wbg.__wbg_texSubImage2D_d2841ded12a8aa66=function(){return A(((a,b,c,e,f,g,h,i,j,k)=>{d(a).texSubImage2D(b>>>L,c,e,f,g,h,i>>>L,j>>>L,d(k))}),arguments)};c.wbg.__wbg_texImage2D_6175916e58c59bc7=function(){return A(((a,b,c,e,f,g,h,i,j,k)=>{d(a).texImage2D(b>>>L,c,e,f,g,h,i>>>L,j>>>L,d(k))}),arguments)};c.wbg.__wbg_texImage2D_07240affd06971e9=function(){return A(((a,b,c,e,f,g,h,i,j,k)=>{d(a).texImage2D(b>>>L,c,e,f,g,h,i>>>L,j>>>L,d(k))}),arguments)};c.wbg.__wbg_colorMask_7cbd7a102954ede9=((a,b,c,e,f)=>{d(a).colorMask(b!==L,c!==L,e!==L,f!==L)});c.wbg.__wbg_colorMask_fba1e2efd891e2ac=((a,b,c,e,f)=>{d(a).colorMask(b!==L,c!==L,e!==L,f!==L)});c.wbg.__wbg_blendEquationSeparate_7ec5e34f066e44f8=((a,b,c)=>{d(a).blendEquationSeparate(b>>>L,c>>>L)});c.wbg.__wbg_blendEquationSeparate_205526dad772d160=((a,b,c)=>{d(a).blendEquationSeparate(b>>>L,c>>>L)});c.wbg.__wbg_blendFuncSeparate_7547ade0a7dfade2=((a,b,c,e,f)=>{d(a).blendFuncSeparate(b>>>L,c>>>L,e>>>L,f>>>L)});c.wbg.__wbg_blendFuncSeparate_fbf93dee3e5ce456=((a,b,c,e,f)=>{d(a).blendFuncSeparate(b>>>L,c>>>L,e>>>L,f>>>L)});c.wbg.__wbg_useProgram_019eb6df066fabf5=((a,b)=>{d(a).useProgram(d(b))});c.wbg.__wbg_useProgram_3683cf6f60939dcd=((a,b)=>{d(a).useProgram(d(b))});c.wbg.__wbg_uniform2f_69ee217590f07278=((a,b,c,e)=>{d(a).uniform2f(d(b),c,e)});c.wbg.__wbg_uniform2f_b6e484a1302ea599=((a,b,c,e)=>{d(a).uniform2f(d(b),c,e)});c.wbg.__wbg_uniform1i_9f94ef0ba6b3cc66=((a,b,c)=>{d(a).uniform1i(d(b),c)});c.wbg.__wbg_uniform1i_d2e61a6a43889648=((a,b,c)=>{d(a).uniform1i(d(b),c)});c.wbg.__wbg_activeTexture_93b4de60af07da9c=((a,b)=>{d(a).activeTexture(b>>>L)});c.wbg.__wbg_activeTexture_799bf1387e911c27=((a,b)=>{d(a).activeTexture(b>>>L)});c.wbg.__wbg_drawElements_3316ee0cd1117c2a=((a,b,c,e,f)=>{d(a).drawElements(b>>>L,c,e>>>L,f)});c.wbg.__wbg_drawElements_a9529eefaf2008bd=((a,b,c,e,f)=>{d(a).drawElements(b>>>L,c,e>>>L,f)});c.wbg.__wbg_deleteShader_7c1222349324b5e2=((a,b)=>{d(a).deleteShader(d(b))});c.wbg.__wbg_deleteShader_9a2f85efe5cb3706=((a,b)=>{d(a).deleteShader(d(b))});c.wbg.__wbindgen_boolean_get=(a=>{const b=d(a);const c=typeof b===T?(b?M:L):2;return c});c.wbg.__wbg_detachShader_a047ade0450ff0bf=((a,b,c)=>{d(a).detachShader(d(b),d(c))});c.wbg.__wbg_detachShader_04abccd441871232=((a,b,c)=>{d(a).detachShader(d(b),d(c))});c.wbg.__wbg_createBuffer_59051f4461e7c5e2=(a=>{const b=d(a).createBuffer();return e(b)?L:k(b)});c.wbg.__wbg_createBuffer_323425af422748ac=(a=>{const b=d(a).createBuffer();return e(b)?L:k(b)});c.wbg.__wbg_bindBuffer_313561e5bc0e533f=((a,b,c)=>{d(a).bindBuffer(b>>>L,d(c))});c.wbg.__wbg_bindBuffer_24f6010e273fa400=((a,b,c)=>{d(a).bindBuffer(b>>>L,d(c))});c.wbg.__wbg_bindVertexArray_8863a216d7b0a339=((a,b)=>{d(a).bindVertexArray(d(b))});c.wbg.__wbg_bindVertexArrayOES_b7d9da7e073aa6b5=((a,b)=>{d(a).bindVertexArrayOES(d(b))});c.wbg.__wbg_bufferData_a11a9f65f31e7256=((a,b,c,e)=>{d(a).bufferData(b>>>L,d(c),e>>>L)});c.wbg.__wbg_bufferData_21334671c4ba6004=((a,b,c,e)=>{d(a).bufferData(b>>>L,d(c),e>>>L)});c.wbg.__wbg_deleteBuffer_2c09d03fa4b0bd08=((a,b)=>{d(a).deleteBuffer(d(b))});c.wbg.__wbg_deleteBuffer_cdc6b9c73f54aff7=((a,b)=>{d(a).deleteBuffer(d(b))});c.wbg.__wbg_deleteTexture_4fcfea73cd8f6214=((a,b)=>{d(a).deleteTexture(d(b))});c.wbg.__wbg_deleteTexture_a883356c5034d482=((a,b)=>{d(a).deleteTexture(d(b))});c.wbg.__wbg_disable_5cf2070641fa2ed7=((a,b)=>{d(a).disable(b>>>L)});c.wbg.__wbg_disable_e02106ca6c7002d6=((a,b)=>{d(a).disable(b>>>L)});c.wbg.__wbg_enable_8965e69c596f0a94=((a,b)=>{d(a).enable(b>>>L)});c.wbg.__wbg_enable_195891416c520019=((a,b)=>{d(a).enable(b>>>L)});c.wbg.__wbg_enableVertexAttribArray_2b0475db43533cf2=((a,b)=>{d(a).enableVertexAttribArray(b>>>L)});c.wbg.__wbg_enableVertexAttribArray_8804480c2ea0bb72=((a,b)=>{d(a).enableVertexAttribArray(b>>>L)});c.wbg.__wbg_getUniformLocation_688976233799a45a=((a,b,c,f)=>{const g=d(a).getUniformLocation(d(b),o(c,f));return e(g)?L:k(g)});c.wbg.__wbg_getUniformLocation_9f6eb60c560a347b=((a,b,c,f)=>{const g=d(a).getUniformLocation(d(b),o(c,f));return e(g)?L:k(g)});c.wbg.__wbg_getAttribLocation_a5a98d5272b01c0d=((a,b,c,e)=>{const f=d(a).getAttribLocation(d(b),o(c,e));return f});c.wbg.__wbg_getAttribLocation_7dbdbad935433494=((a,b,c,e)=>{const f=d(a).getAttribLocation(d(b),o(c,e));return f});c.wbg.__wbg_bindTexture_9cb5c770d1ba2cca=((a,b,c)=>{d(a).bindTexture(b>>>L,d(c))});c.wbg.__wbg_bindTexture_92d6d7f8bff9531e=((a,b,c)=>{d(a).bindTexture(b>>>L,d(c))});c.wbg.__wbg_texParameteri_1f17358e51eb8069=((a,b,c,e)=>{d(a).texParameteri(b>>>L,c>>>L,e)});c.wbg.__wbg_texParameteri_85dad939f62a15aa=((a,b,c,e)=>{d(a).texParameteri(b>>>L,c>>>L,e)});c.wbg.__wbg_vertexAttribPointer_ca11984ee8843c0a=((a,b,c,e,f,g,h)=>{d(a).vertexAttribPointer(b>>>L,c,e>>>L,f!==L,g,h)});c.wbg.__wbg_vertexAttribPointer_316ffe2f0458fde7=((a,b,c,e,f,g,h)=>{d(a).vertexAttribPointer(b>>>L,c,e>>>L,f!==L,g,h)});c.wbg.__wbg_viewport_6ebef187c89e2616=((a,b,c,e,f)=>{d(a).viewport(b,c,e,f)});c.wbg.__wbg_viewport_fad1ce9e18f741c0=((a,b,c,e,f)=>{d(a).viewport(b,c,e,f)});c.wbg.__wbg_call_669127b9d730c650=function(){return A(((a,b)=>{const c=d(a).call(d(b));return k(c)}),arguments)};c.wbg.__wbg_get_4a9aa5157afeb382=((a,b)=>{const c=d(a)[b>>>L];return k(c)});c.wbg.__wbg_self_3fad056edded10bd=function(){return A((()=>{const a=self.self;return k(a)}),arguments)};c.wbg.__wbg_window_a4f46c98a61d4089=function(){return A((()=>{const a=window.window;return k(a)}),arguments)};c.wbg.__wbg_globalThis_17eff828815f7d84=function(){return A((()=>{const a=globalThis.globalThis;return k(a)}),arguments)};c.wbg.__wbg_global_46f939f6541643c5=function(){return A((()=>{const a=global.global;return k(a)}),arguments)};c.wbg.__wbindgen_is_undefined=(a=>{const b=d(a)===J;return b});c.wbg.__wbg_newnoargs_ccdcae30fd002262=((a,b)=>{const c=new Function(o(a,b));return k(c)});c.wbg.__wbg_call_53fc3abd42e24ec8=function(){return A(((a,b,c)=>{const e=d(a).call(d(b),d(c));return k(e)}),arguments)};c.wbg.__wbindgen_memory=(()=>{const a=b.memory;return k(a)});c.wbg.__wbg_buffer_344d9b41efe96da7=(a=>{const b=d(a).buffer;return k(b)});c.wbg.__wbg_newwithbyteoffsetandlength_2dc04d99088b15e3=((a,b,c)=>{const e=new Q(d(a),b>>>L,c>>>L);return k(e)});c.wbg.__wbg_set_dcfd613a3420f908=((a,b,c)=>{d(a).set(d(b),c>>>L)});c.wbg.__wbg_setmultiple_4e25d3b971ac900a=((a,b)=>{d(a).multiple=b!==L});c.wbg.__wbg_new_feb65b865d980ae2=((a,b)=>{try{var c={a:a,b:b};var d=(a,b)=>{const d=c.a;c.a=L;try{return B(d,c.b,a,b)}finally{c.a=d}};const e=new Y(d);return k(e)}finally{c.a=c.b=L}});c.wbg.__wbg_files_0b91078a034a0f7b=(a=>{const b=d(a).files;return e(b)?L:k(b)});c.wbg.__wbg_remove_48288e91662163dc=(a=>{d(a).remove()});c.wbg.__wbg_setonclick_4e9c9187dbc33082=((a,b)=>{d(a).onclick=d(b)});c.wbg.__wbg_new_08236689f0afb357=(()=>{const a=new I();return k(a)});c.wbg.__wbg_buffer_b334b57bee6f611b=(a=>{const b=d(a).buffer;return k(b)});c.wbg.__wbg_push_fd3233d09cf81821=((a,b)=>{const c=d(a).push(d(b));return c});c.wbg.__wbg_new_c728d68b8b34487e=(()=>{const a=new Object();return k(a)});c.wbg.__wbg_set_40f7786a25a9cc7e=function(){return A(((a,b,c)=>{const e=Reflect.set(d(a),d(b),d(c));return e}),arguments)};c.wbg.__wbg_newwithu8arraysequenceandoptions_854056d2c35b489c=function(){return A(((a,b)=>{const c=new Blob(d(a),d(b));return k(c)}),arguments)};c.wbg.__wbg_createObjectURL_d82f2880bada6a1d=function(){return A(((a,c)=>{const e=URL.createObjectURL(d(c));const f=u(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=r;i()[a/X+ M]=g;i()[a/X+ L]=f}),arguments)};c.wbg.__wbg_sethref_a3fde9630423d8ed=((a,b,c)=>{d(a).href=o(b,c)});c.wbg.__wbg_setdownload_0d874703cef6b180=((a,b,c)=>{d(a).download=o(b,c)});c.wbg.__wbg_instanceof_HtmlElement_6f4725d4677c7968=(a=>{let b;try{b=d(a) instanceof HTMLElement}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_setinnerHTML_b089587252408b67=((a,b,c)=>{d(a).innerHTML=o(b,c)});c.wbg.__wbg_settype_ed9a0cf484870612=((a,b,c)=>{d(a).type=o(b,c)});c.wbg.__wbg_setaccept_c88dd3ef66a1bc96=((a,b,c)=>{d(a).accept=o(b,c)});c.wbg.__wbg_instanceof_HtmlButtonElement_6bd3bcb5370764a5=(a=>{let b;try{b=d(a) instanceof HTMLButtonElement}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_setinnerText_1849424c2fdc16ec=((a,b,c)=>{d(a).innerText=o(b,c)});c.wbg.__wbg_measure_aa7a73f17813f708=function(){return A(((a,c,d,e)=>{let f;let g;let h;let i;try{f=a;g=c;h=d;i=e;performance.measure(o(a,c),o(d,e))}finally{b.__wbindgen_free(f,g,M);b.__wbindgen_free(h,i,M)}}),arguments)};c.wbg.__wbg_mark_40e050a77cc39fea=((a,b)=>{performance.mark(o(a,b))});c.wbg.__wbg_log_c9486ca5d8e2cbe8=((a,c)=>{let d;let e;try{d=a;e=c;console.log(o(a,c))}finally{b.__wbindgen_free(d,e,M)}});c.wbg.__wbg_log_aba5996d9bde071f=((a,c,d,e,f,g,h,i)=>{let j;let k;try{j=a;k=c;console.log(o(a,c),o(d,e),o(f,g),o(h,i))}finally{b.__wbindgen_free(j,k,M)}});c.wbg.__wbg_err_5f24e4ca00907ad2=((a,b)=>{console.err(o(a,b))});c.wbg.__wbg_new_9b551002cd49569b=function(){return A((()=>{const a=new FileReader();return k(a)}),arguments)};c.wbg.__wbg_setonload_500a3ab3ebc0147b=((a,b)=>{d(a).onload=d(b)});c.wbg.__wbg_readAsArrayBuffer_07e155f1a3cd4ac2=function(){return A(((a,b)=>{d(a).readAsArrayBuffer(d(b))}),arguments)};c.wbg.__wbg_result_58251a5d230b00f6=function(){return A((a=>{const b=d(a).result;return k(b)}),arguments)};c.wbg.__wbg_downloadToFile_8d9098f4a31783ff=((b,c,d,e)=>{a(o(b,c),o(d,e))});c.wbg.__wbindgen_debug_string=((a,c)=>{const e=v(d(c));const f=u(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=r;i()[a/X+ M]=g;i()[a/X+ L]=f});c.wbg.__wbindgen_throw=((a,b)=>{throw new P(o(a,b))});c.wbg.__wbg_then_89e1c559530b85cf=((a,b)=>{const c=d(a).then(d(b));return k(c)});c.wbg.__wbg_queueMicrotask_e5949c35d772a669=(a=>{queueMicrotask(d(a))});c.wbg.__wbg_then_1bbc9edafd859b06=((a,b,c)=>{const e=d(a).then(d(b),d(c));return k(e)});c.wbg.__wbg_queueMicrotask_2be8b97a81fe4d00=(a=>{const b=d(a).queueMicrotask;return k(b)});c.wbg.__wbindgen_is_function=(a=>{const b=typeof d(a)===R;return b});c.wbg.__wbg_resolve_a3252b2860f0a09e=(a=>{const b=Y.resolve(d(a));return k(b)});c.wbg.__wbg_instanceof_Window_9029196b662bc42a=(a=>{let b;try{b=d(a) instanceof Window}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_data_03708a776af7d2f6=((a,c)=>{const f=d(c).data;var g=e(f)?L:u(f,b.__wbindgen_malloc,b.__wbindgen_realloc);var h=r;i()[a/X+ M]=h;i()[a/X+ L]=g});c.wbg.__wbg_setProperty_b95ef63ab852879e=function(){return A(((a,b,c,e,f)=>{d(a).setProperty(o(b,c),o(e,f))}),arguments)};c.wbg.__wbg_body_674aec4c1c0910cd=(a=>{const b=d(a).body;return e(b)?L:k(b)});c.wbg.__wbg_createElement_4891554b28d3388b=function(){return A(((a,b,c)=>{const e=d(a).createElement(o(b,c));return k(e)}),arguments)};c.wbg.__wbg_getElementById_cc0e0d931b0d9a28=((a,b,c)=>{const f=d(a).getElementById(o(b,c));return e(f)?L:k(f)});c.wbg.__wbg_top_98ff0408c018d25e=(a=>{const b=d(a).top;return b});c.wbg.__wbg_left_23a613d619fb4206=(a=>{const b=d(a).left;return b});c.wbg.__wbg_dataTransfer_bac494821ce31837=(a=>{const b=d(a).dataTransfer;return e(b)?L:k(b)});c.wbg.__wbg_addEventListener_5651108fc3ffeb6e=function(){return A(((a,b,c,e)=>{d(a).addEventListener(o(b,c),d(e))}),arguments)};c.wbg.__wbg_name_a46b2d975591a0b3=((a,c)=>{const e=d(c).name;const f=u(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=r;i()[a/X+ M]=g;i()[a/X+ L]=f});c.wbg.__wbg_get_b383d7f8253c2d61=((a,b)=>{const c=d(a)[b>>>L];return e(c)?L:k(c)});c.wbg.__wbg_getContext_7c5944ea807bf5d3=function(){return A(((a,b,c)=>{const f=d(a).getContext(o(b,c));return e(f)?L:k(f)}),arguments)};c.wbg.__wbg_hidden_736e60e4fd2d186b=(a=>{const b=d(a).hidden;return b});c.wbg.__wbg_focus_dbcbbbb2a04c0e1f=function(){return A((a=>{d(a).focus()}),arguments)};c.wbg.__wbg_instanceof_HtmlInputElement_31b50e0cf542c524=(a=>{let b;try{b=d(a) instanceof HTMLInputElement}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_setvalue_1f95e61cbc382f7f=((a,b,c)=>{d(a).value=o(b,c)});c.wbg.__wbg_ctrlKey_582686fb2263dd3c=(a=>{const b=d(a).ctrlKey;return b});c.wbg.__wbg_metaKey_43193b7cc99f8914=(a=>{const b=d(a).metaKey;return b});c.wbg.__wbg_key_8aeaa079126a9cc7=((a,c)=>{const e=d(c).key;const f=u(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=r;i()[a/X+ M]=g;i()[a/X+ L]=f});c.wbg.__wbg_hash_a1a795b89dda8e3d=function(){return A(((a,c)=>{const e=d(c).hash;const f=u(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=r;i()[a/X+ M]=g;i()[a/X+ L]=f}),arguments)};c.wbg.__wbg_userAgent_2053026e2b1e0c7e=function(){return A(((a,c)=>{const e=d(c).userAgent;const f=u(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=r;i()[a/X+ M]=g;i()[a/X+ L]=f}),arguments)};c.wbg.__wbg_appendChild_51339d4cde00ee22=function(){return A(((a,b)=>{const c=d(a).appendChild(d(b));return k(c)}),arguments)};c.wbg.__wbg_touches_8338f31b10bd7975=(a=>{const b=d(a).touches;return k(b)});c.wbg.__wbg_changedTouches_60ab7fa55837664f=(a=>{const b=d(a).changedTouches;return k(b)});c.wbg.__wbg_get_d6c4e69528650af4=((a,b)=>{const c=d(a)[b>>>L];return e(c)?L:k(c)});c.wbg.__wbg_getExtension_77909f6d51d49d4d=function(){return A(((a,b,c)=>{const f=d(a).getExtension(o(b,c));return e(f)?L:k(f)}),arguments)};c.wbg.__wbg_getParameter_55b36a787dbbfb74=function(){return A(((a,b)=>{const c=d(a).getParameter(b>>>L);return k(c)}),arguments)};c.wbg.__wbg_getExtension_088d115a16ecbd7d=function(){return A(((a,b,c)=>{const f=d(a).getExtension(o(b,c));return e(f)?L:k(f)}),arguments)};c.wbg.__wbg_getParameter_bfab7f0b00c9d7fb=function(){return A(((a,b)=>{const c=d(a).getParameter(b>>>L);return k(c)}),arguments)};c.wbg.__wbg_document_f7ace2b956f30a4f=(a=>{const b=d(a).document;return e(b)?L:k(b)});c.wbg.__wbg_open_7a2a86bf6285507d=function(){return A(((a,b,c,f,g)=>{const h=d(a).open(o(b,c),o(f,g));return e(h)?L:k(h)}),arguments)};c.wbg.__wbg_setTimeout_eb1a0d116c26d9f6=function(){return A(((a,b,c)=>{const e=d(a).setTimeout(d(b),c);return e}),arguments)};c.wbg.__wbindgen_closure_wrapper527=((a,b,c)=>{const d=w(a,b,Z,x);return k(d)});c.wbg.__wbindgen_closure_wrapper836=((a,b,c)=>{const d=w(a,b,Z,y);return k(d)});c.wbg.__wbindgen_closure_wrapper958=((a,b,c)=>{const d=w(a,b,Z,z);return k(d)});c.wbg.__wbindgen_closure_wrapper3130=((a,b,c)=>{const d=w(a,b,101,z);return k(d)});c.wbg.__wbindgen_closure_wrapper3951=((a,b,c)=>{const d=w(a,b,140,x);return k(d)});return c});var g=(()=>{if(f===K||f.byteLength===L){f=new Float64Array(b.memory.buffer)};return f});var y=((a,c)=>{try{const f=b.__wbindgen_add_to_stack_pointer(-W);b.wasm_bindgen__convert__closures__invoke0_mut__hb0825f3b5602fa64(f,a,c);var d=i()[f/X+ L];var e=i()[f/X+ M];if(e){throw q(d)}}finally{b.__wbindgen_add_to_stack_pointer(W)}});var p=(a=>{if(a<132)return;c[a]=j;j=a});var w=((a,c,d,e)=>{const f={a:a,b:c,cnt:M,dtor:d};const g=(...a)=>{f.cnt++;const c=f.a;f.a=L;try{return e(c,f.b,...a)}finally{if(--f.cnt===L){b.__wbindgen_export_2.get(f.dtor)(c,f.b)}else{f.a=c}}};g.original=f;return g});var u=((a,b,c)=>{if(c===J){const c=s.encode(a);const d=b(c.length,M)>>>L;n().subarray(d,d+ c.length).set(c);r=c.length;return d};let d=a.length;let e=b(d,M)>>>L;const f=n();let g=L;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==L){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,M)>>>L;const b=n().subarray(e+ g,e+ d);const f=t(a,b);g+=f.written};r=g;return e});var n=(()=>{if(m===K||m.byteLength===L){m=new Q(b.memory.buffer)};return m});var o=((a,b)=>{a=a>>>L;return l.decode(n().subarray(a,a+ b))});var F=((a,c)=>{b=a.exports;H.__wbindgen_wasm_module=c;f=K;h=K;m=K;b.__wbindgen_start();return b});import{downloadToFile as a}from"./snippets/turing-machine-206e2362c10fe35c/assets/utils.js";let b;const c=new I(128).fill(J);c.push(J,K,!0,!1);let f=K;let h=K;let j=c.length;const l=typeof TextDecoder!==N?new TextDecoder(O,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw P(`TextDecoder not available`)}};if(typeof TextDecoder!==N){l.decode()};let m=K;let r=L;const s=typeof TextEncoder!==N?new TextEncoder(O):{encode:()=>{throw P(`TextEncoder not available`)}};const t=typeof s.encodeInto===R?((a,b)=>s.encodeInto(a,b)):((a,b)=>{const c=s.encode(a);b.set(c);return {read:a.length,written:c.length}});export default H;export{G as initSync}