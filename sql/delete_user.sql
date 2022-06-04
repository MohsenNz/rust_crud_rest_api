DELETE FROM testing.users 
WHERE phone_number = $1
RETURNING $table_fields;
