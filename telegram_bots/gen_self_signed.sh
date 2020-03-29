if [ -z "$1" ]; then
	echo "use $0 <ip-address>"
else
	ip=$1
	openssl req -x509 \
		-subj "/C=CZ/CN=$ip" \
		-days 1000 \
		-addext "subjectAltName = IP:$ip" \
		-newkey rsa:2048 \
		-keyout telegram_bot_backend.key -out telegram_bot_backend.pem \
		-nodes
fi
