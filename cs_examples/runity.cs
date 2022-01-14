using System.Runtime.InteropServices;
using System;
using UnityEngine;
using TMPro;
using System.Collections.Generic;
using UnityEngine.Scripting;
using System.Text;
using System.Reflection;

[assembly: Preserve]

/*   This script acts as a sort of buffer
    between the DLL (used for scripting)
    and the unity engine. 
    
    This script should be attached to the game object you want the script to affect.
*/

namespace runity_test
{
    public class Runity : MonoBehaviour
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
        DestroyDelegate destroy;

        // A string builder to go from pointer to string, GC free. Only supports 128 chars
        StringBuilder stringBuilder = new StringBuilder(128, 128);

        // An object pool to avoid calling Find on gameobjects every frame
        Dictionary<StringBuilder, UnityEngine.GameObject> objectPool = new Dictionary<StringBuilder, UnityEngine.GameObject>();

        // We use these booleans to check if we should run the respective unity functions.
        // This is so we can check collisions conditionally.
        bool runStart;
        bool runUpdate;

        // We don't have an optional destroy function - this must exist.

        /* Import our functions (start, update, destroy and awake) as delegates, as they will be pointers to the functions
         * since we want to load them dynamically at runtime 
         */

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        private delegate DataStruct StartDelegate(DataStruct data);

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        private delegate DataStruct UpdateDelegate(DataStruct data);

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        private delegate DataStruct DestroyDelegate(DataStruct data);


        /* Define the structs to use to interface with rust with. This gives
         * the option to implement safe, rust compatiable datatypes which
         * we can't guarantee with unity */

        [StructLayout(LayoutKind.Sequential)]
        public struct String
        {
            public IntPtr ptr;
            public Int32 len;
        }

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
            public String tag;
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
        public delegate GameObject FindGameObjectWithTagDelegate(String tag); // This delegate acts as FindGameObjectWithTag


        /* Run built-in unity functions */

        // Start is called before the first frame update
        void Start()
        {
            // Load the DLL. This is important, as the DLL must be loaded before we can call any functions
            DLLPool.LoadDLL(DLLName);

            (Delegate startFunction, IntPtr startPtr) = DLLPool.LoadFunctionFromDLL(DLLName, "start", typeof(StartDelegate));

            if (startPtr != IntPtr.Zero)
            {
                runStart = true;
                Debug.Log("Start function loaded");
                start = (StartDelegate)startFunction;
            }
            else
            {
                Debug.LogWarning("Start function not loaded");
            }


            (Delegate updateFunction, IntPtr updatePtr) = DLLPool.LoadFunctionFromDLL(DLLName, "update", typeof(UpdateDelegate));

            if (updatePtr != IntPtr.Zero)
            {
                runUpdate = true;
                Debug.Log("Update function loaded");
                update = (UpdateDelegate)updateFunction;
            }
            else
            {
                Debug.LogWarning("Update function not loaded");
            }

            (Delegate destroyFunction, IntPtr destroyPtr) = DLLPool.LoadFunctionFromDLL(DLLName, "destroy", typeof(DestroyDelegate));

            if (destroyPtr != IntPtr.Zero)
            {
                Debug.Log("Destroy function loaded");
                destroy = (DestroyDelegate)destroyFunction;
            }
            else
            {
                Debug.LogError("Destroy function not loaded");
            }

            // We now assign our delegates to point to our functions.

            // Initialize values with no value (should be set in Rust's start function)
            m_transform = new Transform { };
            m_gameObject = new GameObject { };
            dataStruct = new DataStruct { };

            m_time = new Time { };

            // Now start

            if (runStart)
            {
                m_transform.position = new Vector3 { x = 0, y = 0, z = 0 };
                m_transform.rotation = new Quaternion { x = 0, y = .25f, z = 0, w = 1.0f };


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
            dataStruct = destroy(dataStruct);
            DLLPool.UnloadDLL(DLLName);
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
        public GameObject GetGameObjectFromTag(String tag)
        {
            GameObject gameObject = new GameObject { transform = new Transform { position = new Vector3 { x = 0, y = 0, z = 0, } } };

            // We check if the object with its tag is not already pooled. If it is, we make sure it hasn't been destroyed and then take it from the pool.
            // otherwise, we load it and add it to the pool

           // This will convert the pointer to a stringbuilder,
           // which is a 0 alloc and fast alternative to a string.
           // This is what we use to index the dictionary.

            stringBuilder.Clear();

            for (int i = 0; i < tag.len; i++)
            {
                stringBuilder.Append((char)Marshal.ReadByte(tag.ptr + i));
            }

            // This long, complex code basically checks that the object exists in the pool. If it doesn't, we add it. 
            //
            // It will find the gameobject, find its transforms, then add them to the gameobject that it returns. This should be callable from rust.
            //
            // If it doesn't find the gameobject, it defaults to zero for pos and rot. This is simply a fallback.
            UnityEngine.GameObject foundObj;

            if (objectPool.TryGetValue(stringBuilder, out foundObj))
            {
                // Make sure it isn't a false positive, and return the value.
                // If it is, find it and store it.
                if (foundObj == null)
                {
                    foundObj = UnityEngine.GameObject.FindGameObjectWithTag(stringBuilder.ToString());
                    objectPool.Add(stringBuilder, foundObj);
                }

                Transform transform = new Transform
                {
                    position = new Vector3 { x = foundObj.transform.position.x, y = foundObj.transform.position.y, z = foundObj.transform.position.z },
                    rotation = new Quaternion { x = foundObj.transform.rotation.x, y = foundObj.transform.rotation.y, z = foundObj.transform.rotation.z, w = foundObj.transform.rotation.w }
                };
                gameObject.transform = transform;
                gameObject.tag = tag; // This assigns a borrowed string to the gameObject's tag. This is fairly unsafe behavior in rust. Therefore, the GameObject returned 
                                      // from this function is read-only.
            }
            else
            {
                // Our requested object hasn't been found, so now we search it. If it doesn't exist, return
                // a default value. If it does, store it and return the value.
                foundObj = UnityEngine.GameObject.FindGameObjectWithTag(stringBuilder.ToString());
                if (foundObj == null)
                {
                    Debug.LogWarning("Warning: Tag -> " + stringBuilder.ToString()  + " was not found. Falling back to default transform. (all values zeroed) ");
                    Transform transform = new Transform
                    {
                        position = new Vector3 { x = 0, y = 0, z = 0 },
                        rotation = new Quaternion { x = 0, y = 0, z = 0, w = 0 }
                    };
                    gameObject.transform = transform;
                    gameObject.tag = tag;
                }
                else
                {
                    Transform transform = new Transform
                    {
                        position = new Vector3 { x = foundObj.transform.position.x, y = foundObj.transform.position.y, z = foundObj.transform.position.z },
                        rotation = new Quaternion { x = foundObj.transform.rotation.x, y = foundObj.transform.rotation.y, z = foundObj.transform.rotation.z, w = foundObj.transform.rotation.w }
                    };
                    gameObject.transform = transform;
                    gameObject.tag = tag;
                    objectPool.Add(stringBuilder, foundObj);
                }
            }

            return gameObject;
        }
    }

}