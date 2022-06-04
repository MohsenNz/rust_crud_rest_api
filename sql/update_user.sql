UPDATE testing.users
SET first_name = $2, last_name = $3, birthday = $4, contacts = $5
WHERE phone_number = $1
RETURNING $table_fields;
