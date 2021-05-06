/*
This source code file is distributed subject to the terms of the GNU Affero General Public License.
A copy of this license can be found in the at the root of this project.
*/
CREATE TABLE IF NOT EXISTS categories (
  c_id SERIAL PRIMARY KEY,
  genre VARCHAR NOT NULL
);
