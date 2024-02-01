#!/bin/bash

psql -U postgres < /create_role.sql
psql -U ragnarok < /db/pg.sql
psql -U ragnarok < /db/alter_itemdb_add_script_compilation_result_column.sql