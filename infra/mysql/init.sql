CREATE DATABASE IF NOT EXISTS vihaan_db;
USE vihaan_db;
 
create table users (
id bitint Auto_incriment PRIMARY KEY,
  email Varchar(255) NOT NULL Unique
);
