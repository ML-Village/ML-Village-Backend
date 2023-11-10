# FROM python:3.9-slim AS prover

# WORKDIR /app

# RUN apt-get update && \
#   apt-get install -y --no-install-recommends g++ libgmp3-dev && \
#   pip install ecdsa==0.18.0 \
#   bitarray==2.7.3 \
#   fastecdsa==2.2.3 \
#   sympy==1.11.1 \
#   typeguard==2.13.3 \
#   cairo-lang==0.11.0

# # this is node
# COPY node /app/node 
# #this is python
# COPY lambdaworks_stark_platinum /app/stark_prover

# WORKDIR /app/node
# RUN npm install

# CMD [ "npm", "run", "dev" ]

# Stage 1: Node.js and npm setup
FROM node:18 AS node_stage

WORKDIR /app

COPY node/package.json /app/
COPY node/package-lock.json /app/

RUN npm install

# Stage 2: Python setup
FROM python:3.9-slim AS python_stage

WORKDIR /app

RUN apt-get update && \
  apt-get install -y --no-install-recommends g++ libgmp3-dev

COPY lambdaworks_stark_platinum /app/stark_prover

RUN pip install ecdsa==0.18.0 \
  bitarray==2.7.3 \
  fastecdsa==2.2.3 \
  sympy==1.11.1 \
  typeguard==2.13.3 \
  cairo-lang==0.11.0

# Stage 3: Final image with both Node.js and Python
FROM node_stage AS final_stage

COPY /node /app/node
COPY --from=node_stage /app/node_modules /app/node_modules
COPY --from=python_stage /app/stark_prover /app/stark_prover

WORKDIR /app/node

EXPOSE 3000

CMD [ "npm", "run", "dev" ]
