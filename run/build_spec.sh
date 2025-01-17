#!/bin/bash -x

function subkey() {
    cargo run --release --package subkey -- "$@"
}

if [ "$1" = newkeys ]; then
	# generate keys
	rawbabe_nuc=`subkey generate`
	rawbabe_sveit=`subkey generate`
	rawbabe_tw=`subkey generate`

	# keep private phrases
	privnuc=`echo "$rawbabe_nuc" | head -1 | sed -s 's/Secret phrase:\s*//'`
	privsveit=`echo "$rawbabe_sveit" | head -1 | sed -s 's/Secret phrase:\s*//'`
	privtw=`echo "$rawbabe_tw" | head -1 | sed -s 's/Secret phrase:\s*//'`

	# save them locally
	echo "nuc private passphrase" > ~/privkeys
	echo $privnuc >> ~/privkeys
	echo "sveit private passphrase" >> ~/privkeys
	echo $privsveit >> ~/privkeys
	echo "tw private passphrase" >> ~/privkeys
	echo $privtw >> ~/privkeys

	# get babe pubkey
	export babenuc=`echo "$rawbabe_nuc" | tail -1 | sed -s 's/\s*SS58 Address:\s*//'`
	export babesveit=`echo "$rawbabe_sveit" | tail -1 | sed -s 's/\s*SS58 Address:\s*//'`
	export babetw=`echo "$rawbabe_tw" | tail -1 | sed -s 's/\s*SS58 Address:\s*//'`

	rawgran_nuc=`subkey inspect --scheme Ed25519 "$privnuc"`
	rawgran_sveit=`subkey inspect --scheme Ed25519 "$privsveit"`
	rawgran_tw=`subkey inspect --scheme Ed25519 "$privtw"`

	export grannuc=`echo "$rawgran_nuc" | tail -1 | sed -s 's/\s*SS58 Address:\s*//'`
	export gransveit=`echo "$rawgran_sveit" | tail -1 | sed -s 's/\s*SS58 Address:\s*//'`
	export grantw=`echo "$rawgran_tw" | tail -1 | sed -s 's/\s*SS58 Address:\s*//'`

	echo "export babenuc=$babenuc" > pub_keys
	echo "export grannuc=$grannuc" >> pub_keys
	echo "export babesveit=$babesveit" >> pub_keys
	echo "export gransveit=$gransveit" >> pub_keys
	echo "export babetw=$babetw" >> pub_keys
	echo "export grantw=$grantw" >> pub_keys
fi

source pub_keys

# create spec from keys
envsubst < template_smly3.json > smly3.json

# to build the raw spec, if needed
../target/release/substrate-node build-spec --chain smly3.json --raw --disable-default-bootnode > raw_smly3.json

# then you might at some point want to move the newly generated raw spec to the ../substrate/bin/node/cli/res 
# directory and recompile for the new spec to be included in the binary

