fn main() {
    //definicion de variables no mutables
    //let x = 5;
    //println!("el valor es {}",x);
    //esto genera error ya que la variable no esta definida como mutable
    //^^ cannot assign twice to immutable variable

    // x = 7;
    //println!("el valor es {}",x);

    //para solucionar este error se tiene que agregar la palabra reservada mut
    let mut x = 7;
    println!("el valor es {}",x);

    x = 9;
    println!("el valor es {}", x);


}
