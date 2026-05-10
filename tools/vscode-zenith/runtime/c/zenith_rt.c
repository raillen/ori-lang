#include "runtime/c/zenith_rt.h"
#include "runtime/c/zenith_rt_templates.h"
#include "runtime/c/zenith_collections_generic.h"

#include <ctype.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <setjmp.h>
#include <stdatomic.h>
#include <sys/stat.h>
#include <time.h>
#include <math.h>
#include <limits.h>
#ifdef _WIN32
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#include <winsock2.h>
#include <ws2tcpip.h>
#include <direct.h>
#include <io.h>
#include <process.h>
#include <windows.h>
#include <conio.h>
#else
#include <dirent.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <netdb.h>
#include <sys/select.h>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <termios.h>
#include <unistd.h>
#include <sys/wait.h>
#include <dlfcn.h>
#ifdef __APPLE__
#include <mach-o/dyld.h>
#endif
#endif

#ifdef _WIN32
typedef SOCKET zt_socket_handle;
#define ZT_NET_INVALID_SOCKET INVALID_SOCKET
#else
typedef int zt_socket_handle;
#define ZT_NET_INVALID_SOCKET (-1)
#endif

#include "zenith_rt_core.c"
#include "zenith_rt_outcome.c"
#include "zenith_collections_generic.c"
#include "zenith_rt_memory.c"

#include "zenith_rt_host.c"
#include "zenith_rt_json.c"
#include "zenith_rt_format.c"
#include "zenith_rt_math.c"

#include "zenith_rt_scalar.c"
#include "zenith_rt_random.c"
#include "zenith_rt_encoding.c"
#include "zenith_rt_net.c"

#include "zenith_rt_http.c"

#include "zenith_rt_borealis.c"
#include "zenith_rt_path.c"

/* R2.M1 (T2.9): specialized collections moved to a sibling unity file. */
#include "zenith_collections_rt.c"

/* R3.M4: Generic dyn dispatch runtime implementation */

#include "zenith_rt_dyn.c"
