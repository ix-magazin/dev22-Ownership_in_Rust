////////////////////////////////////////////////////////////////////////////////////// 
// Listing 1: Dynamische Speicherreservierung und Befüllung von drei Ganzahlen in C //
////////////////////////////////////////////////////////////////////////////////////// 

#include <stdio.h>
#include <stdlib.h>

int main() {
    int* ptr;
    int n = 3;
    // Speicher anlegen
    ptr = (int*) malloc(n * sizeof(int));

    // Befüllen
    for(int i = 0; i < n; i++) {
        ptr[i] = i;
    }
  
    for(int i = 0; i < n; i++) {
        printf("%d ", ptr[i]);
    }

    printf("\n");

    // Freigeben
    free(ptr);
}
  

////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Listing 2: Dynamische Speicherreservierung und Befüllung für drei Ganzzahlen in Rust mit dem vec-Makro //
////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    // Speicher für vec wird hier angelegt
    let vec = vec![0, 1, 2]; 

    println!("{:?}", vec);
} // vec wird hier freigegeben.


//////////////////////////////////////////////////////////////////////////////////////////////////////////
// Listing 3: Ein fehlerhafter Versuch, auf Daten zuzugreifen, die bereits einen anderen Besitzer haben //
//////////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let vec = vec![0, 1, 2]; // Speicher für vec wird hier angelegt
    let another_vec = vec; // Zuweisung
    println!("{:?}", vec); // vec ist nicht mehr Besitzer der Daten
}


/////////////////////////////////////////////////////////////////////////
// Listing 4: Die Funktion print_vec fordert den Besitz des Vektors an //
/////////////////////////////////////////////////////////////////////////

fn main() {
    let vec = vec![0, 1, 2];
    print_vec(vec);
}

fn print_vec(vec: Vec<i32>) {
    println!("{:?}", vec);
}


////////////////////////////////////////////////////////////////////////////////////////////////////////
// Listing 5: Die Funktion print_vec fordert eine Referenz an – sie lässt sich beliebig oft übergeben //
////////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let vec = vec![0, 1, 2];
    //&vec erstellt eine Referenz auf vec
    print_vec(&vec);
    print_vec(&vec);
    print_vec(&vec);
    print_vec(&vec);
}

fn print_vec(vec: &Vec<i32>) { // Erfordert eine Referenz auf einen Vektor
    println!("{:?}", vec);
}


////////////////////////////////////////////////////////////////////
// Listing 6: Die Referenz zum Vektor in einer Variable speichern //
////////////////////////////////////////////////////////////////////

fn main() {
    let vec = vec![0, 1, 2];
    let vec_ref = &vec;
    print_vec(vec_ref);
    print_vec(vec_ref);
    print_vec(vec_ref);
    print_vec(vec_ref);
}

/////////////////////////////////////////////////////////////////////////
// Listing 7: Mehrere Shared References auf den selben Speicherbereich //
/////////////////////////////////////////////////////////////////////////

fn main() {
    let vec = vec![0, 1, 2];
    let vec_ref = &vec;
    let vec_ref_2 = &vec;
    print_vec(vec_ref);
    print_vec(vec_ref_2);
}


////////////////////////////////////////
// Listing 8: Veränderbare Referenzen //
////////////////////////////////////////

fn main() {
    // mut kennzeichnet Veränderung
    let mut vec = vec![0, 1, 2];
    // &mut kennzeichnet eine veränderbare Referenz
    change_vec(&mut vec);
    // Eine normale Referenz
    print_vec(&vec);
}

// change_vec fordert eine veränderbare Referenz an
fn change_vec(vec: &mut Vec<i32>) {
    vec.push(3);
}

// print_vec nicht
fn print_vec(vec: &Vec<i32>) {
    println!("{:?}", vec);
}


/////////////////////////////////////////////////////////////////////////////
// Listing 9: Shared und mutable references im gleichen Ausführungskontext //
/////////////////////////////////////////////////////////////////////////////

fn main() {
    let mut vec = vec![0, 1, 2];
    let mut_ref = &mut vec;
    let vec_ref = &vec;
    change_vec(mut_ref);
    print_vec(vec_ref);
}


///////////////////////////////////////////////////////////////
// Listing 10: Die Nutzung von shared und mutable references //
///////////////////////////////////////////////////////////////

fn main() {
    let mut vec = vec![0, 1, 2];
    // Nutzung von mut_ref startet hier
    let mut_ref = &mut vec;
    // Nutzung von vec_ref startet hier
    let vec_ref = &vec;
    change_vec(mut_ref);
    // Nutzung von mut_ref endet hier
    print_vec(vec_ref);
    // Nutzung von vec_ref endet hier
}


/////////////////////////////////////////////////////////////////////////////////
// Listing 11: Keine Kreuzung in der Nutzung von shared und mutable references //
/////////////////////////////////////////////////////////////////////////////////

fn main() {
    let mut vec = vec![0, 1, 2];
    // Fluss von mut_ref startet hier
    let mut_ref = &mut vec;
    change_vec(mut_ref);
    // Fluss von mut_ref endet hier
    // Fluss von vec_ref startet hier
    let vec_ref = &vec;
    print_vec(vec_ref);
    // Fluss von vec_ref endet hier
}
