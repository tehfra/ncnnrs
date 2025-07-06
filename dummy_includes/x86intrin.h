/* Dummy x86intrin.h to prevent MinGW headers from failing
 * This header is included by winnt.h but we don't want actual intrinsics
 * for bindgen cross-compilation to Windows targets
 */
#ifndef _X86INTRIN_H_INCLUDED
#define _X86INTRIN_H_INCLUDED

/* This dummy header satisfies MinGW's requirement for x86intrin.h
 * without actually including any intrinsics functionality */

#endif /* _X86INTRIN_H_INCLUDED */
