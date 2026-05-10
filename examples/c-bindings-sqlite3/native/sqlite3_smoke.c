#include <sqlite3.h>

#include "runtime/c/zenith_rt.h"

zt_int zt_sqlite3_memory_exec(zt_text *sql) {
    sqlite3 *db = NULL;
    char *error_message = NULL;
    const char *sql_text = "";
    int rc;
    int close_rc;

    if (sql != NULL && sql->data != NULL) {
        sql_text = sql->data;
    }

    rc = sqlite3_open(":memory:", &db);
    if (rc != SQLITE_OK) {
        if (db != NULL) {
            sqlite3_close(db);
        }
        return (zt_int)(rc == SQLITE_OK ? 1 : rc);
    }

    rc = sqlite3_exec(db, sql_text, NULL, NULL, &error_message);
    if (error_message != NULL) {
        sqlite3_free(error_message);
    }

    close_rc = sqlite3_close(db);
    if (rc != SQLITE_OK) {
        return (zt_int)rc;
    }
    return (zt_int)close_rc;
}
