// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "plugin.h"

enum c_log_level {
    None = 0,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
};

struct c_log_topic {
    uint32_t version;
    const char *topic;
    enum c_log_level level;
    bool has_custom_level;
};

struct c_log_methods {
	uint32_t version;

	void (*log) (void *object,
		     enum c_log_level level,
		     const char *file,
		     int line,
		     const char *func,
		     const char *fmt, ...);

	void (*logv) (void *object,
		      enum c_log_level level,
		      const char *file,
		      int line,
		      const char *func,
		      const char *fmt,
		      va_list args);

	void (*logt) (void *object,
		     enum c_log_level level,
		     const struct c_log_topic *topic,
		     const char *file,
		     int line,
		     const char *func,
		     const char *fmt, ...);

	void (*logtv) (void *object,
		      enum c_log_level level,
		      const struct c_log_topic *topic,
		      const char *file,
		      int line,
		      const char *func,
		      const char *fmt,
		      va_list args);

	/* Deprecated */
	void (*topic_init) (void *object, struct c_log_topic *topic);
};

struct c_log {
	struct c_interface iface;
	uint32_t level;
};

extern void rust_logt(void *object,
		      enum c_log_level level,
		      const struct c_log_topic *topic,
		      const char *file,
		      uint32_t line,
		      const char *func,
		      const char *buf);

void impl_logtv(void *object,
		enum c_log_level level,
		const struct c_log_topic *topic,
		const char *file,
		int line,
		const char *func,
		const char *fmt,
		va_list args)
{
	/* Because we can't meaningfully deal with variadic arguments in Rust,
	 * we snprintf the arguments into a buffer on the stack, so we have a
	 * two step render: one for building the user-provided log line on the
	 * stack, and then for actually building the final log output with all
	 * the additional metadata. */

	char buf[16384];

	vsnprintf(buf, sizeof(buf), fmt, args);

	rust_logt(object, level, topic, file, line, func, buf);
}

void impl_logt(void *object,
		enum c_log_level level,
		const struct c_log_topic *topic,
		const char *file,
		int line,
		const char *func,
		const char *fmt, ...)
{
	va_list args;

	va_start(args, fmt);
	impl_logtv(object, level, topic, file, line, func, fmt, args);
	va_end(args);
}

void impl_logv(void *object,
		enum c_log_level level,
		const char *file,
		int line,
		const char *func,
		const char *fmt,
		va_list args)
{
	impl_logtv(object, level, NULL, file, line, func, fmt, args);
}

void impl_log(void *object,
		enum c_log_level level,
		const char *file,
		int line,
		const char *func,
		const char *fmt, ...)
{
	va_list args;

	va_start(args, fmt);
	impl_logtv(object, level, NULL, file, line, func, fmt, args);
	va_end(args);
}

static struct c_log_methods log_funcs = {
	.version = 1,
	.log = impl_log,
	.logv = impl_logv,
	.logt = impl_logt,
	.logtv = impl_logtv,
	.topic_init = NULL,
};

struct c_log *c_log_from_impl(void *impl, enum c_log_level level)
{
	struct c_log *log = calloc(1, sizeof(struct c_log));

	log->iface.version = 1;
	log->iface.type_ = "Spa:Pointer:Interface:Log";
	log->iface.cb.cb = &log_funcs;
	log->iface.cb.data = impl;
	log->level = level;

	return log;
}

void c_log_free(struct c_log *log) {
	free(log);
}
