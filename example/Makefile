java_run: lib
	javac LongToDoubleSwissTable.java && java -Djava.library.path=mylib/target/debug/ LongToDoubleSwissTable

.PHONY: lib

javah:
	javah LongToDoubleSwissTable

lib:
	cd mylib && cargo build
