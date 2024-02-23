let a = 0
for (let i = 0; i < 5; i++) {
    for (let k = 0; k < i; k++) {
        console.log('a: %d', a);
        a = a + 1
    }
    console.log()
}
