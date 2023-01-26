movs r0, #0 @2000  
movs r1, #1 @2101 
.goto:
movs r2, #20 @2214
cmp r0, r1 @4288
bMI .then1 @d4ff
b .endif1 @e7ff 
.then1:
rsbs r2, r2, #0 @4252 
.endif1:
cmp r2, r1 @428a
bLT .then2 @dbff
b .endif2 @e000 
.then2:
movs r0, #50 @2032 
b .goto @e7f4 
.endif2:
adds r3, r0, r2 @1883
@r3 value should be 70, 46
