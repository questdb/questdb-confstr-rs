#pragma once

#include <stdint.h>

#if defined(__cplusplus)
extern "C" {
#endif

typedef struct questdb_conf_str questdb_conf_str;

struct questdb_conf_str_parse_err {
    const char* msg;
    size_t msg_len;
    size_t pos;
};

typedef struct questdb_conf_str_parse_err questdb_conf_str_parse_err;

void questdb_conf_str_parse_err_free(questdb_conf_str_parse_err* err);

questdb_conf_str* questdb_conf_str_parse(
    const char* str,
    size_t len,
    questdb_conf_str_parse_err** err_out);

const char* questdb_conf_str_service(
    const questdb_conf_str* conf_str,
    size_t* len_out);

const char* questdb_conf_str_get(
    const questdb_conf_str* conf_str,
    const char* key,
    size_t key_len,
    size_t* val_len_out);

void questdb_conf_str_free(questdb_conf_str* str);

#if defined(__cplusplus)
}
#endif
