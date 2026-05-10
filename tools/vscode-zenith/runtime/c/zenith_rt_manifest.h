#ifndef ZENITH_NEXT_RUNTIME_C_ZENITH_RT_MANIFEST_H
#define ZENITH_NEXT_RUNTIME_C_ZENITH_RT_MANIFEST_H

#define ZT_RUNTIME_UNITY_SOURCE "runtime/c/zenith_rt.c"

#define ZT_RUNTIME_DEPENDENCIES(X) \
    X("runtime/c/zenith_rt_manifest.h") \
    X("runtime/c/zenith_rt.c") \
    X("runtime/c/zenith_rt.h") \
    X("runtime/c/zenith_rt_templates.h") \
    X("runtime/c/zenith_rt_core.c") \
    X("runtime/c/zenith_collections_generic.c") \
    X("runtime/c/zenith_collections_generic.h") \
    X("runtime/c/zenith_collections_rt.c") \
    X("runtime/c/zenith_rt_memory.c") \
    X("runtime/c/zenith_rt_outcome.c") \
    X("runtime/c/zenith_rt_host.c") \
    X("runtime/c/zenith_rt_format.c") \
    X("runtime/c/zenith_rt_path.c") \
    X("runtime/c/zenith_rt_math.c") \
    X("runtime/c/zenith_rt_scalar.c") \
    X("runtime/c/zenith_rt_random.c") \
    X("runtime/c/zenith_rt_encoding.c") \
    X("runtime/c/zenith_rt_json.c") \
    X("runtime/c/zenith_rt_net.c") \
    X("runtime/c/zenith_rt_http.c") \
    X("runtime/c/zenith_rt_borealis.c") \
    X("runtime/c/zenith_rt_dyn.c")

#endif
