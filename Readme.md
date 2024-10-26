# ShullerBot

This bot can find some cool images from rule34 by your reqwest!

**Note** */porno* command only available on NSFW channels
## RU
<img width="880" src="https://github.com/user-attachments/assets/450c899a-21f0-492a-ba20-4570fcf0e7fc">
<img width="1015"  src="https://github.com/user-attachments/assets/9f625652-f5cb-4e6c-bc1e-7c451632f4ad">
<img src="https://github.com/user-attachments/assets/c053225d-5cda-4111-856f-54c5a0909499">

## EN
<img width="880"  src="https://github.com/user-attachments/assets/82d596ac-af43-472a-8ab7-a8cb8fb96aa8">
<img width="1015"  src="https://github.com/user-attachments/assets/e011d2c4-a377-47c5-8098-83cb52005b2b">
<img src="https://github.com/user-attachments/assets/cf0f7614-baa7-410b-9842-3ed7150fe7c8">

# Docker
```sh
docker run -d \
  -e DS_TOKEN=YOUR_TOKEN \
  ghcr.io/towinok/shuller_bot:latest
```

# Docker-compose
```yml
services:
  shuller_bot:
    image: ghcr.io/towinok/shuller_bot:latest
    environment:
      - DS_TOKEN=YOUR_TOKEN
```
