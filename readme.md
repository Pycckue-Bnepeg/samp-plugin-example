# SA:MP Plugin Example
There is a simple Memcached plugin showing how to use samp-sdk in Rust.

## Include file
```C
native Memcached_Connect(const address[]);
native Memcached_Get(connection, const key[], &value);
native Memcached_GetString(connection, const key[], value[], size=sizeof(value));
native Memcached_Set(connection, const key[], value, expire);
native Memcached_SetString(connection, const key[], const value[], expire);
native Memcached_Delete(connection, const key[]);
native Memcached_Increment(connection, const key[], value);
```

## Pawn example
```C
main() {
	new con = Memcached_Connect("memcache://localhost:11211");
	
	Memcached_Set(con, "foo", 228, 0);
	
	new value = 0;
	new string[256];

	if (Memcached_Get(con, "foo", value)) {
		printf("%d", value);
	} else {
		printf("cannot get a value");
	}

	Memcached_Delete(con, "foo");
	Memcached_Set(con, "foo", 1, 0);
	Memcached_Increment(con, "foo", 5);
	Memcached_Get(con, "foo", value);
	Memcached_SetString(con, "string", "some value", 0);
	Memcached_GetString(con, "string", string);

	printf("%d %s", value, string);
}
```