# Root Document

The root documentation showed in top of page.
1. each line starting with `///*`.


# Function Documentation

The documentation for function and function like macro
1. Started with `/**`.
2. Ended with `*/`.
3. Description started with `* @brief`. Must only containt 1 brief. Can be multiline
4. Parameter description started with `* @param` then followed by `variable name`. Can be multiline.
5. Note description started with `* @note`. Must only containt 1 note. Can be multiline.
6. Function like macro name will be followed by `(macro)`.

# Struct Documentation
1. Started with `/**`.
2. Ended with `*/`.
3. Description started with `* @brief`. Must only containt 1 brief. Can be multiline
4. Note description started with `* @note`. Must only containt 1 note. Can be multiline.
5. Render: 
```c
struct MyStruct{
int field;
};
```
title: struct MyStruct. src: same as source

6. Render: 
```c
typedef struct _MyStruct MyStruct;
struct _MyStruct{
int field;
};
```
title: MyStruct. src: same as source

7. Render: 
```c
typedef struct {
int field;
}MyStruct;
```
title: MyStruct. src: same as source

8. Render: 
```c
typedef struct _MyStruct MyStruct;
```
title: MyStruct. src:
```c
typedef struct _MyStruct {
/* PRIVATE FIELD*/
}MyStruct;
```

