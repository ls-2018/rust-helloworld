fn v() {
    let s1 = String::from("aaa");
    let s2 = String::from("bbb");
    let s3 = String::from("ccc");
    let s4 = String::from("ddd");

    let v = vec![s1, s2, s3, s4];
    v[0].clone();
    let a = &v[0]; // 明确a只获得v中第一个元素的引用

    let v = vec![1, 2, 3, 4];
    let a = &v[0]; // 明确a只获得v中第一个元素的引用
}
