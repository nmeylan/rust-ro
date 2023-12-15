#!/bin/bash

psql -U postgres < /create_role.sql
psql -U ragnarok < /pg.sql