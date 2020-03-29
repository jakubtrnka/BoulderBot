if [ -n "$1" ] ; then
	accesstoken=$1
	curl "https://api.telegram.org/bot${accesstoken}/getWebhookInfo" | jq
else
	echo "Use $0 <accesstoken>"
fi
