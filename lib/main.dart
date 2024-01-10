import 'package:flutter/material.dart';
import 'package:flutter_rust_demo/src/rust/api/simple.dart';
import 'package:flutter_rust_demo/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  Future<List<String>> fetchData() async {
    // You can replace this with the actual logic to fetch data
    List<String> object =
        await getToken(input: 'คอร์สเรียนออนไลน์เพิ่มทักษะยุคดิจิทัล') ?? [""];
    // String object1 = await getcurrentdir();
    // Future.delayed(Duration(seconds: 2));
    // if object!
    // Simulating data fetching
    print(object);
    return object;
  }

  Future<String> fetchData222() async {
    // You can replace this with the actual logic to fetch data
    // List<String> object =
    // await getToken(input: 'คอร์สเรียนออนไลน์เพิ่มทักษะยุคดิจิทัล') ?? [""];
    String object1 = await getcurrentdir();
    // Future.delayed(Duration(seconds: 2));
    // if object!
    // Simulating data fetching
    print(object1);
    return object1;
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: SingleChildScrollView(
            child: Column(
              children: [
                Text('${hello(a: 'JIN')}`\n'),
                Text('Result: `${greet(name: "PATH:")}`\n'),
                Text('fetchData:${fetchData()}'),
                Text('getToken:${getToken(input: 'ผมก็ว่าพูดยาก')}'),
                FutureBuilder(
                  future: fetchData(),
                  builder: (context, AsyncSnapshot<List<String>> snapshot) {
                    if (snapshot.connectionState == ConnectionState.done) {
                      if (snapshot.hasData) {
                        return Text(snapshot.data!.join('\n'));
                      } else if (snapshot.hasError) {
                        return Text('Error: ${snapshot.error}');
                      } else {
                        return Text('Data is null');
                      }
                    } else {
                      return CircularProgressIndicator();
                    }
                  },
                ),
              ],
            ),
          ),
        ),
        floatingActionButton: FloatingActionButton(
          onPressed: () async {
            print(fetchData());
            // List<String> getDataFromRust = await getToken(input: 'ผมก็ว่าพูดยาก ');
            // print(getDataFromRust);
            // print(myRustFunction(a: 'Hello'));
          },
          child: const Icon(Icons.navigation),
        ),
      ),
    );
  }
}
