# RustでDeepLearning 1章
ゼロから作るDeep LearningをRustでやってみる

1章は基本的なプログラムの使い方について説明している. ここではrustの行列演算ライブラリ `ndarray` と 画像処理系ライブラリ `image`　で遊びます.
ゼロから作るDeep Learningに合わせるならmatplotlib相当なのですが,どれもまだまだ枯れていないように見えていないのと,グラフ描画はそこまでないので,この２つに絞りました.
最初にndarrayの使い方について説明します.


# Chapter 1

## ndarray
### データの生成
[ソース](https://github.com/rust-ndarray/ndarray/blob/master/src/free_functions.rs#L222)を見ると以下のメソッドを使って,
二次元のndarrayが生成できます.

```rust
pub fn arr2<A: Clone, V: FixedInitializer<Elem = A>>(xs: &[V]) -> Array2<A>
where
    V: Clone,
{
    Array2::from(xs.to_vec())
}
```
rustの文法はなれないと難しいので、少し解説します.
このメソッドの引数の型 `&[V]`,復帰値の型は `Array2<A>`です.
`&` は参照で, Vは`FixedInitializer<Elem = A`>
`Elem` はndarrayのTypeで, `FixedInitializer`はそれを含むTraitです.
`FixedInitializer`はサイズ固定の初期化処理
`arr2`の直後等の `< >`はジェネリクスですね.
`Elem`であれば特に問題がないので、intでもfloatでも特に気にせず同じ文法で扱えるようです.

実際に生成します.

```rust
use ndarray::arr2;

let a = arr2(&[[1, 2, 3],
               [4, 5, 6]]);
println!("{:?}", a);
```

これは `arr2` という文字からもわかるように二次元のものです.
少し見た限り,データの生成の場合は以下がよく使うものに見えました.

`array!` , `Array::from_vec` , `arr2`

次元を明記した方がわかりやすいと思うので,ここではarr2を使います.

### 値の取得

二次元配列の場合は
`data[[i, j]]`でアクセスできるようです.
実際には3種類あり,範囲外だと`None`になるようです.

```rust
use ndarray::arr2;

let a = arr2(&[[1., 2.],
               [3., 4.]]);

assert!(
    a.get([0, 1]) == Some(&2.) &&
    a.get((0, 2)) == None &&
    a[(0, 1)] == 2. &&
    a[[0, 1]] == 2.
);
```

### 値のループ
思ったより複雑です.
pythonのenumerate相当がかんたんにつくれるのかわかりませんが,基本は以下でいけるようです.

- `iter`, `iter_mut` :配列の内部値の参照を取得?します.実際には`Iter`オブジェクトが返ります.
正直`Iter`オブジェクトの挙動が掴みきれませんが, .next()で次の値をOption型を得られます.

```rust
    let a = arr3(&[[[ 50,  1,  2],
                [ 3,  4,  5]],
               [[ 6,  7,  8],
                [ 9, 10, 11]]]);
    println!("{:?}", a.iter().next().unwrap());
```
- `genrows`,  `genrows_mut` は列をループします

```rust
    let mut a = Array::zeros((10, 10));
    for mut row in a.genrows_mut() {
        println!("{:}", row);
        row.fill(1.);
        println!("{:}", row);
    }
    println!("{:}", a);
```

zipで複数の配列をループさせます.
ちなみに`a.cols()`はmutableでないと動きません.(なんで?)
どれも型を推定するのがかなり難しい.

```rust
    use ndarray::Zip;
     // 1. Loop over the rows of a 2D array
    let mut a = Array::zeros((10, 10));
    for mut row in a.genrows_mut() {
        row.fill(1.);
    }

    let mut b = Array::zeros(a.rows());
    println!("a is {:?}", a.rows());

    Zip::from(a.genrows())
        .and(&mut b)
        .apply(|a_row, b_elt| {
            *b_elt = a_row[a.cols() - 1] + a_row[0];
        });
    println!("b {:?}", b);

    let mut c = Array::zeros(10);
    Zip::from(&mut c)
        .and(&b)
        .apply(|c_elt, &b_elt| {
            *c_elt = b_elt + 1.;
        });
    println!("c {:?}", c);

    use ndarray::Array2;
    type M = Array2<f64>;
    let mut a = M::zeros((12, 8));
    let b = M::from_elem(a.dim(), 1.);
    let c = M::from_elem(a.dim(), 2.);
    let d = M::from_elem(a.dim(), 3.);


    Zip::from(&mut a)
        .and(&b)
        .and(&c)
        .and(&d)
        .apply(|w, &x, &y, &z| {
            *w += x + y * z;
        });
    println!("{:?}", a);
```

### スライス

スライスは比較的シンプルにそのままでよい.
```rust
let a = arr2(&[[1., 2.],
            [3., 4.]]);

prinltlin("{:?}", a.slice(s![.., 0..1])
```

### 行列の和、差、積、要素積
ここから実際に行列だと思って演算をします.
これらは簡単です.


```rust
let a = arr2(&[[1., 2.],
            [3., 4.]]);
let b = arr2(&[[5., 6.],
            [3., 4.]]);
let c = a.add(&b);
let d = c.sub(&b);
let e = d.dot(&b);
let f = e.mul(&b); // 要素積
println!("f {}", f);
```

となります.

## 画像
Imaage
いつかやる.


# Chapeter2 パーセプトロン

ここでは二層のシンプルなパーセプトロンについて説明します.
ここで二層のパーセプトロンとは具体的には$x_1, x_2$という入力に対して,$w_1, w_2, \theta$というパラメータを定め,
出力を以下で定めたものとします.
$$
y = \left\{
\begin{array}{ll}
0 & w_1x_1 + w_2 x_2 < \theta \\
1 & w_1x_2 + w_2x_2 \ge \theta
\end{array}
\right.
$$

二層のパーセプトロンで
- `AND`
- `OR`
- `NAND`
が実現でき,逆に
- `XOR`
が二層のパーセプトロンが実現できないことを示します.

まず`AND`ですが
$w_1 = w_2 = 0.5$とし,$\theta = 0.7$とすればよいです.
実際計算してみると$(x_1, x_2)$が(0,0),(1,0),(0,1)のときは高々0.5しか行かず,(1,1)のときは1となります.
$\theta$の条件を考えるとこれが`AND`になることがわかります.

また`OR`は$w_1 = w_2 = 0.5$とし,$\theta = 0.3$とすればよいです.
`NAND`は`AND`を反転させたものなので,
$w_1 = w_2 = -0.5$とし,$\theta = - 0.7$とすればよいです.
とすればよいです.

一方で`XOR`は実現できません.なぜなら
$(1, 0),(0, 1)$でともに
$w_1x_2 + w_2x_2 \ge \theta$を満たしている.
つまり,$w_1 \ge \theta, w_2 \ge \theta$の時,($\theta > 0$の時)$w_1 + w_2 \ge \theta$となります.
逆に$\theta \le 0$の問$0 \ge \theta$となります.
なので
$(1, 0),(0, 1)$でともに出力が1なら$(1,1),(0, 0)$少なくとも一方が1になることがわかります.
なので、これを実現することは不可能です.

ただし、これを三層のパーセプトロンにすれば実現できます.

$$
\begin{array}{ll}
s_1 =  NAND(x_1, x_2) \\
s_2 =  OR(x_1, x_2) \\
y = AND(s_1, s_2)
\end{array}
$$
とすればNANDを実現することができます,

これは三層のパーセプトロンが二層では表せない関数を作れるということで真に表現力が豊かであることがわかります.


では実際にこれらをrustで実装してみましょう.
