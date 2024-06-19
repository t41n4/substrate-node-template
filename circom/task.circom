pragma circom 2.1.6;

include "poseidon.circom";
include "comparators.circom";

// include "https://github.com/0xPARC/circom-secp256k1/blob/master/circuits/bigint.circom";


template Task(n) {
    signal input a;
    signal input b[n]; 
    signal output isContained;

    var i;
    var sum;
    component eq[n];
    component hash_list[n];
    component hash_num = Poseidon(1);

    component zero = Poseidon(1);
    zero.inputs[0] <== 0;

    for (i = 0; i < n; i++) {
        hash_list[i] = Poseidon(1);
        hash_list[i].inputs[0] <== b[i];
    }
    
    hash_num.inputs[0] <== a;
    for (i = 0; i < n; i++) {
        eq[i] = IsEqual();
        eq[i].in[0] <== hash_num.out;
        eq[i].in[1] <== hash_list[i].out;
        sum += eq[i].out;
    }

    component gt = GreaterThan(8);
    gt.in[0] <== sum;
    gt.in[1] <== 0;

    isContained <== zero.out + gt.out;
}

component main = Task(1000);