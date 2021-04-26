using System.Runtime.InteropServices;
using System;
using UnityEngine;
using TMPro;
using System.Collections.Generic;
using UnityEngine.Scripting;

[assembly: Preserve]

/*   This script acts as a sort of buffer
    between the DLL (used for scripting)
    and the unity engine. 
    
    This script should be attached to the game object you want the script to affect.
*/

namespace runity_test
{
    public class runity: MonoBehaviour
    {
        /* Variables */

        // Our DLL name
        public string DLLName = "runity.dll";


        // A copy of our position
        public Vector3 position;

        // A copy of our rotation
        public Quaternion rotation;

        // Some variables to store per update info
        public GameObject m_gameObject;
        public Transform m_transform;
        public DataStruct dataStruct;
        public Time m_time;

        // Our function pointers, so we can recycle them rather than waste
        // processing time reloading the DLL
        StartDelegate start;
        UpdateDelegate update;

        // An object pool to avoid calling Find on gameobjects every frame
        Dictionary<string, UnityEngine.GameObject> objectPool = new Dictionary<string, UnityEngine.GameObject>();

        // We use these booleans to check if we should run the respective unity functions.
        // This is so we can check collisions conditionally.
        bool runStart;
        bool runUpdate;

        /* Import our functions (start, update and awake) as delegates, as they will be pointers to the functions
         * since we want to load them dynamically at runtime 
         */

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        private delegate DataStruct StartDelegate(DataStruct data);

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        private delegate DataStruct UpdateDelegate(DataStruct data);


        /* Define the structs to use to interface with rust with. This gives
         * the option to implement safe, rust compatiable datatypes which
         * we can't guarantee with unity */

        [StructLayout(LayoutKind.Sequential)]
        public struct Vector3
        {
            public float x;
            public float y;
            public float z;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct Quaternion
        {
            public float x;
            public float y;
            public float z;
            public float w;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct Transform
        {
            public Vector3 position;
            public Quaternion rotation;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct GameObject
        {
            public IntPtr tag;
            public Transform transform;
            public FindGameObjectWithTagDelegate GetGameObjectByTag;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct DataStruct
        {
            public Transform transform;
            public GameObject gameObject;
            public Time time;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct Time
        {
            public float deltaTime;
            public float fixedDeltaTime;
            public float fixedTime;
            public float fixedUnscaledDeltaTime;
            public float fixedUnscaledTime;
            public float frameCount;
            public float maximumDeltaTime;
            public float maximumParticleDeltaTime;
            public float realTimeSinceStartup;
            public float smoothDeltaTime;
            public float time;
            public float timeScale;
            public float timeSinceLevelLoad;
            public float unscaledDeltaTime;
            public float unscaledTime;
        }

        /* Define our delegates, which are callbacks to functions we want to use 
         * in rust */
        public delegate GameObject FindGameObjectWithTagDelegate(IntPtr name); // This delegate acts as FindGameObjectWithTag


        /* Run built-in unity functions */

        void Awake()
        {
            (Delegate startFunction, IntPtr startPtr) = DLLPool.LoadFunctionFromDLL(DLLName, "start", typeof(StartDelegate));

            if (startPtr != IntPtr.Zero)
            {
                Debug.Log("Running function!");
                runStart = true;
                start = (StartDelegate)startFunction;
            }


            (Delegate updateFunction, IntPtr updatePtr) = DLLPool.LoadFunctionFromDLL(DLLName, "update", typeof(UpdateDelegate));

            if (updatePtr != IntPtr.Zero)
            {
                Debug.Log("Running function!");
                runUpdate = true;
                update = (UpdateDelegate)updateFunction;
            }



            // We now assign our delegates to point to our functions.

            // Initialize values with no value (should be set in Rust's start function)
            m_transform = new Transform { };
            m_gameObject = new GameObject { };
            dataStruct = new DataStruct { };

            m_time = new Time { };
        }


        // Start is called before the first frame update
        void Start()
        {
            if (runStart)
            {
                m_transform.position = new Vector3 { x = 0, y = 0, z = 0 };
                m_transform.rotation = new Quaternion { x= 0, y = .25f, z = 0, w = 1.0f };


                m_gameObject.transform = m_transform;
                m_gameObject.GetGameObjectByTag = new FindGameObjectWithTagDelegate(GetGameObjectFromTag);

                dataStruct.transform = m_transform;
                dataStruct.gameObject = m_gameObject;

                SetTime();

                dataStruct.time = m_time;

                dataStruct = start(dataStruct);

                UnityEngine.Time.fixedDeltaTime = dataStruct.time.fixedDeltaTime;
                UnityEngine.Time.timeScale = dataStruct.time.timeScale;

                m_gameObject = dataStruct.gameObject;
                m_transform = dataStruct.transform;
                position = m_transform.position;
                rotation = m_transform.rotation;

                transform.position = new UnityEngine.Vector3(position.x, position.y, position.z);
                transform.rotation = new UnityEngine.Quaternion(rotation.x, rotation.y, rotation.z, rotation.w);
            }
        }

        // Update is called once per frame
        void Update()
        {
            if (runUpdate)
            {
                m_gameObject.transform = m_transform;

                dataStruct.transform = m_transform;
                dataStruct.gameObject = m_gameObject;

                SetTime();

                dataStruct.time = m_time;

                dataStruct = update(dataStruct);

                UnityEngine.Time.fixedDeltaTime = dataStruct.time.fixedDeltaTime;
                UnityEngine.Time.timeScale = dataStruct.time.timeScale;

                m_gameObject = dataStruct.gameObject;
                m_transform = dataStruct.transform;
                position = m_transform.position;
                rotation = m_transform.rotation;

                transform.position = new UnityEngine.Vector3(position.x, position.y, position.z);
                transform.rotation = new UnityEngine.Quaternion(rotation.x, rotation.y, rotation.z, rotation.w);
            }
        }

        // This function releases all our pointers to remain safe
        private void OnDestroy()
        {
            // This is VERY important, we must free and release the link before we exit!
            NativeMethods.Free(m_gameObject.tag);
            Debug.Log("Released pointers properly");
        }

        /// <summary>
        /// Update time - should be run *before* submitting data to rust
        /// </summary>
        void SetTime()
        {
            m_time.deltaTime = UnityEngine.Time.deltaTime;
            m_time.fixedDeltaTime = UnityEngine.Time.fixedDeltaTime;
            m_time.fixedTime = UnityEngine.Time.fixedTime;
            m_time.fixedUnscaledDeltaTime = UnityEngine.Time.fixedUnscaledDeltaTime;
            m_time.fixedUnscaledTime = UnityEngine.Time.fixedUnscaledTime;
            m_time.frameCount = UnityEngine.Time.frameCount;
            m_time.maximumDeltaTime = UnityEngine.Time.maximumDeltaTime;
            m_time.maximumParticleDeltaTime = UnityEngine.Time.maximumParticleDeltaTime;
            m_time.realTimeSinceStartup = UnityEngine.Time.realtimeSinceStartup;
            m_time.smoothDeltaTime = UnityEngine.Time.smoothDeltaTime;
            m_time.time = UnityEngine.Time.time;
            m_time.timeScale = UnityEngine.Time.timeScale;
            m_time.timeSinceLevelLoad = UnityEngine.Time.timeSinceLevelLoad;
            m_time.unscaledDeltaTime = UnityEngine.Time.unscaledDeltaTime;
            m_time.unscaledTime = UnityEngine.Time.unscaledTime;
        }


        /* We can now define the functions we want to expose to rust */

        // This method converts `FindGameObjectWithTag` into our custom defined structs
        public GameObject GetGameObjectFromTag(IntPtr tag)
        {
            GameObject gameObject = new GameObject { transform = new Transform { position = new Vector3 { x = 0, y = 0, z = 0, } } };

            // We check if the object with its tag is not already pooled. If it is, we make sure it hasn't been destroyed and then take it from the pool.
            // otherwise, we load it and add it to the pool

            string string_tag = Marshal.PtrToStringAnsi(tag); // try and save some processing time by storing the tag as a string beforehand.

            // This long, complex code basically checks that the object exists in the pool. If it doesn't, we add it. 
            //
            // It will find the gameobject, find its transforms, then add them to the gameobject that it returns. This should be callable from rust.
            //
            // If it doesn't find the gameobject, it defaults to zero for pos and rot. This is simply a fallback.
            if (objectPool.ContainsKey(string_tag))
            {
                UnityEngine.GameObject foundObj = objectPool[string_tag];
                if (foundObj == null)
                {
                    foundObj = UnityEngine.GameObject.FindGameObjectWithTag(string_tag);
                    Transform transform = new Transform { 
                        position = new Vector3 { x = foundObj.transform.position.x, y = foundObj.transform.position.y, z = foundObj.transform.position.z },
                        rotation = new Quaternion { x = foundObj.transform.rotation.x, y = foundObj.transform.rotation.y, z = foundObj.transform.rotation.z, w = foundObj.transform.rotation.w} 
                        };
                    gameObject.transform = transform;
                    gameObject.tag = tag;
                    objectPool.Add(string_tag, foundObj);
                }
                else
                {
                    Transform transform = new Transform { 
                        position = new Vector3 { x = foundObj.transform.position.x, y = foundObj.transform.position.y, z = foundObj.transform.position.z },
                        rotation = new Quaternion { x = foundObj.transform.rotation.x, y = foundObj.transform.rotation.y, z = foundObj.transform.rotation.z, w = foundObj.transform.rotation.w} 
                        };                   
                    gameObject.transform = transform;
                    gameObject.tag = tag;
                }
            }
            else
            {
                UnityEngine.GameObject foundObj = UnityEngine.GameObject.FindGameObjectWithTag(string_tag);
                if (foundObj == null)
                {
                    Debug.LogWarning("Warning: Tag -> " + string_tag + " was not found. Falling back to default transform. ");
                    Transform transform = new Transform { 
                        position = new Vector3 { x = 0, y = 0, z = 0 },
                        rotation = new Quaternion { x = 0, y = 0, z = 0, w = 0 }
                        };
                    gameObject.transform = transform;
                    gameObject.tag = tag;
                }
                else
                {
                    Transform transform = new Transform { 
                        position = new Vector3 { x = foundObj.transform.position.x, y = foundObj.transform.position.y, z = foundObj.transform.position.z },
                        rotation = new Quaternion { x = foundObj.transform.rotation.x, y = foundObj.transform.rotation.y, z = foundObj.transform.rotation.z, w = foundObj.transform.rotation.w} 
                        };
                    gameObject.transform = transform;
                    gameObject.tag = tag;
                    objectPool.Add(string_tag, foundObj);
                }
            }

            return gameObject;
        }
    }

}