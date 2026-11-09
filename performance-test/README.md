# Performance tests

### Prerequisites
1. [rust](../README.md#Rust)
2. docker

Make sure you have docker installed using this command:
```bash script
docker --version
```
or install it: https://docs.docker.com/engine/install/

3. curl
   
Make sure you have curl installed using this command:
```bash script
curl --version
```
or install it: https://curl.se/docs/install.html   
4. k6

Make sure you have k6 installed using this command:
```bash script
k6 --version
```
or install it: https://grafana.com/docs/k6/latest/set-up/install-k6/


## Running k6 performance-test
1. Create docker image of pdfgenrs
```bash
cd ..
docker build -t pdfgenrs .
docker run --rm -it -p 8080:8080 pdfgenrs
```

2. Then you can run the performance test
```bash script
cd performance-test
k6 run -e BASE_URL=http://localhost:8080 script.js
```