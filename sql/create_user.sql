INSERT INTO testing.users(phone_number, first_name, last_name, birthday, contacts)
VALUES ($1, $2, $3, $4, $5)
RETURNING $table_fields;
