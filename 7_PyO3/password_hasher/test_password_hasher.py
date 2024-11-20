import password_hasher

# Хешування паролю
hashed = password_hasher.hash_password("my_secure_password")
print("Hashed password:", hashed)

# Перевірка паролю
is_valid = password_hasher.verify_password("my_secure_password", hashed)
print("Is valid:", is_valid)
