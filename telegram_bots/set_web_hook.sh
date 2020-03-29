if [ -n "$1" ] && [ -n "$2" ] && [ -n "$3" ]; then
	bot_name=$1
	accesstoken=$2
	ip_address=$3
	curl -XPOST "https://api.telegram.org/bot${accesstoken}/setWebhook" \
		-F "url=https://${ip_address}/${bot_name}/${accesstoken}" \
		-F "certificate=@telegram_bot_backend.pem" | jq
else
	echo "Use $0 <bot_name> <accesstoken> <ip:port>"
fi
