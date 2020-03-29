if [ -n "$1" ] && [ -n "$2" ] && [ -n "$3" ]; then
	bot_name=$1
	accesstoken=$2
	ip_address=$3
	curl -v -X POST -H "Content-Type: application/json" -H "Cache-Control: no-cache"  -d '{
	"update_id":10000,
	"callback_query":{
	  "id": "4382bfdwdsb323b2d9",
	  "from":{
	     "last_name":"Test Lastname",
	     "type": "private",
	     "id":1111111,
	     "first_name":"Test Firstname",
	     "username":"Testusername"
	  },
	  "data": "Data from button callback",
	  "inline_message_id": "1234csdbsk4839"
	}
	}' "https://${ip_address}/${bot_name}/${accesstoken}" --cacert telegram_bot_backend.pem | jq
else
	echo "Use $0 <botname> <accesstoken> <ip:port>"
fi
