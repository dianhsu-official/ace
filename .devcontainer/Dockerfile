FROM rust

RUN apt-get update
RUN apt-get install -y xdg-utils 
RUN rustup component add rustfmt
RUN echo 'export PATH=$PATH:/workspaces/ace/target/debug' >> /root/.bashrc