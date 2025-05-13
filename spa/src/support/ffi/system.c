// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

#include <stdarg.h>
#include <sys/ioctl.h>

int impl_ioctl(void *object, int fd, unsigned long request, ...)
{
	va_list args;
	long arg;
	int ret;

	va_start(args, request);
	arg = va_arg(args, long);
	ret = ioctl(fd, request, args);
	va_end(args);

	return ret;
}
